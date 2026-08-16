//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1677/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1677<F: Float>(t41520: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F) -> (F, F) {
    let t88462 = F::cast_from(0.24722222222222222222e-1_f64) * t77559 - F::cast_from(0.74166666666666666668e-1_f64) * t77561 + F::cast_from(0.13734567901234567901e-1_f64) * t77499 - F::cast_from(0.16481481481481481482e-1_f64) * t63453 + F::cast_from(0.49444444444444444445e-1_f64) * t63459 + t41520 + F::cast_from(0.2225e0_f64) * t88085 - F::cast_from(0.33375e0_f64) * t88089 + F::cast_from(0.55625000000000000001e-1_f64) * t88093 + F::cast_from(0.74166666666666666668e-1_f64) * t88097 - F::cast_from(0.24722222222222222222e-1_f64) * t63464;
    let t88475 = F::cast_from(0.12361111111111111111e-1_f64) * t77505 - F::cast_from(0.27469135802469135803e-1_f64) * t88104 - F::cast_from(0.92708333333333333333e-2_f64) * t88108 - F::cast_from(0.49444444444444444444e-1_f64) * t77507 + F::cast_from(0.74166666666666666668e-1_f64) * t77509 + F::cast_from(0.12361111111111111111e0_f64) * t88114 - F::cast_from(0.61805555555555555555e-1_f64) * t88118 - F::cast_from(0.22249999999999999999e0_f64) * t88122 + F::cast_from(0.22249999999999999999e0_f64) * t88126 - F::cast_from(0.18541666666666666666e-1_f64) * t88130 - F::cast_from(0.24722222222222222222e-1_f64) * t88134 + F::cast_from(0.38456790123456790123e-1_f64) * t51978;
    (t88462, t88475)
}
