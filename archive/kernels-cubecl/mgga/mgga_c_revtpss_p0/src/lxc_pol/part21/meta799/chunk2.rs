//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2895/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2895<F: Float>(t41267: F, t41275: F, t41672: F, t51921: F, t51923: F, t51927: F, t51932: F, t51935: F, t51937: F, t51940: F, t51942: F, t51945: F) -> F {
    let t52562 = F::cast_from(0.69463333333333333334e-1_f64) * t51921 + F::cast_from(0.92617777777777777778e-1_f64) * t51923 - F::cast_from(0.104195e0_f64) * t51927 - F::cast_from(0.13892666666666666667e0_f64) * t51932 - F::cast_from(0.34731666666666666667e-1_f64) * t51935 - F::cast_from(0.41678000000000000001e0_f64) * t51937 - F::cast_from(0.125034e1_f64) * t51940 + F::cast_from(0.125034e1_f64) * t51942 + F::cast_from(0.250068e1_f64) * t51945 + t41672 - F::cast_from(0.41678000000000000001e0_f64) * t41267 + F::cast_from(0.41678000000000000001e0_f64) * t41275;
    t52562
}
