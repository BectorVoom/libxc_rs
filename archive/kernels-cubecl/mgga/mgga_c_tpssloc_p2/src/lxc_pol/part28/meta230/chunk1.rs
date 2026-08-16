//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1005/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1005<F: Float>(t28: F, t1302: F, t2: F, t1081: F, t5178: F, t584: F, t5177: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t5181 = t1302 * t2;
    let t5185 = piecewise3::<F>(t29, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5178 * t1081 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5181 * t584);
    let t5187 = t5177 / F::cast_from(2.0_f64) + t5185 / F::cast_from(2.0_f64);
    (t5181, t5187)
}
