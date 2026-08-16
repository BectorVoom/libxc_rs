//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2425/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2425<F: Float>(t49378: F, t59657: F, t60163: F, t60168: F, t60173: F, t68536: F, t68541: F, t68545: F, t68549: F, t68552: F, t68556: F, t68563: F) -> F {
    let t69105 = F::cast_from(0.20839e0_f64) * t68536 - F::cast_from(0.34731666666666666667e-1_f64) * t68541 + F::cast_from(0.250068e1_f64) * t68545 - F::cast_from(0.187551e1_f64) * t68549 - F::cast_from(0.125034e1_f64) * t68552 + F::cast_from(0.62517e0_f64) * t68556 + F::cast_from(0.20839e0_f64) * t60163 + F::cast_from(0.69463333333333333335e0_f64) * t60168 - F::cast_from(0.34731666666666666667e0_f64) * t60173 - F::cast_from(0.45908888888888888888e0_f64) * t59657 - F::cast_from(0.13892666666666666667e0_f64) * t68563 + t49378;
    t69105
}
