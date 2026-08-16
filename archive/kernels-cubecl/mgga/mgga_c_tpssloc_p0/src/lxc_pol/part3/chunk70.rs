//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 70/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk70<F: Float>(t40: F, t52: F, t73: F, t194: F, t76: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t195 = t73 * t73;
    let t196 = piecewise3::<F>(t146, t194, t195);
    let t197 = t76 * t76;
    let t198 = piecewise3::<F>(t150, t194, t197);
    let t200 = t196 / F::cast_from(2.0_f64) + t198 / F::cast_from(2.0_f64);
    (t195, t197, t200)
}
