//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1238/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1238<F: Float>(t28: F, t3672: F, t517: F, t5966: F, t6312: F, t157: F, t6311: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t6318 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3672 * t6312 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t517 * t5966);
    let t6320 = (t6311 + t6318) * t157;
    t6320
}
