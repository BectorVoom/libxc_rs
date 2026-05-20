//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2883/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2883<F: Float>(t51973: F, t41361: F, t41363: F, t41369: F, t41520: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F) -> F {
    let t52337 = F::cast_from(0.12361111111111111111e-1_f64) * t51973;
    let t52345 = F::new(0.2225e0) * t51849 - F::cast_from(0.61805555555555555555e-2_f64) * t51853 - F::cast_from(0.27469135802469135803e-1_f64) * t51858 + F::cast_from(0.55625000000000000001e-1_f64) * t51863 + F::cast_from(0.55625000000000000001e-1_f64) * t51867 + F::cast_from(0.18541666666666666667e-1_f64) * t51871 - F::cast_from(0.22249999999999999999e0_f64) * t51875 + t41520 + F::new(0.11125e0) * t51961 - F::cast_from(0.30902777777777777778e-1_f64) * t51965 + F::cast_from(0.92708333333333333334e-2_f64) * t51967 - F::cast_from(0.92708333333333333333e-2_f64) * t51971 - t52337 + F::cast_from(0.96141975308641975309e-2_f64) * t51978 + F::cast_from(0.28842592592592592593e-1_f64) * t41361 + F::cast_from(0.24722222222222222222e-1_f64) * t41363 - F::cast_from(0.12361111111111111111e-1_f64) * t41369 + F::new(0.11125e0) * t52028 + F::cast_from(0.12361111111111111111e0_f64) * t52031 + F::new(0.55625e-1) * t52033;
    t52345
}
