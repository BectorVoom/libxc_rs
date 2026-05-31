//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2919/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2919<F: Float>(t11223: F, t379: F, t51973: F, t41361: F, t41363: F, t41369: F, t42013: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F) -> (F, F) {
    let t52927 = t11223 * t379;
    let t52946 = F::cast_from(0.11111111111111111111e-1_f64) * t51973;
    let t52954 = F::cast_from(0.2e0_f64) * t51849 - F::cast_from(0.55555555555555555555e-2_f64) * t51853 - F::cast_from(0.24691358024691358025e-1_f64) * t51858 + F::cast_from(0.50000000000000000001e-1_f64) * t51863 + F::cast_from(0.50000000000000000001e-1_f64) * t51867 + F::cast_from(0.16666666666666666667e-1_f64) * t51871 - F::cast_from(0.19999999999999999999e0_f64) * t51875 + t42013 + F::cast_from(0.99999999999999999999e-1_f64) * t51961 - F::cast_from(0.27777777777777777777e-1_f64) * t51965 + F::cast_from(0.83333333333333333334e-2_f64) * t51967 - F::cast_from(0.83333333333333333333e-2_f64) * t51971 - t52946 + F::cast_from(0.86419753086419753086e-2_f64) * t51978 + F::cast_from(0.25925925925925925926e-1_f64) * t41361 + F::cast_from(0.22222222222222222222e-1_f64) * t41363 - F::cast_from(0.11111111111111111111e-1_f64) * t41369 + F::cast_from(0.1e0_f64) * t52028 + F::cast_from(0.11111111111111111111e0_f64) * t52031 + F::cast_from(0.5e-1_f64) * t52033;
    (t52927, t52954)
}
