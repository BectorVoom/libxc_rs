//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1703/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1703<F: Float>(t42078: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F) -> (F, F) {
    let t89144 = F::cast_from(0.39511111111111111112e-1_f64) * t77559 - F::cast_from(0.11853333333333333334e0_f64) * t77561 + F::cast_from(0.21950617283950617284e-1_f64) * t77499 - F::cast_from(0.26340740740740740742e-1_f64) * t63453 + F::cast_from(0.79022222222222222224e-1_f64) * t63459 + t42078 + F::cast_from(0.35560000000000000001e0_f64) * t88085 - F::cast_from(0.53340000000000000002e0_f64) * t88089 + F::cast_from(0.88900000000000000002e-1_f64) * t88093 + F::cast_from(0.11853333333333333334e0_f64) * t88097 - F::cast_from(0.39511111111111111112e-1_f64) * t63464;
    let t89157 = F::cast_from(0.19755555555555555556e-1_f64) * t77505 - F::cast_from(0.43901234567901234568e-1_f64) * t88104 - F::cast_from(0.14816666666666666667e-1_f64) * t88108 - F::cast_from(0.79022222222222222224e-1_f64) * t77507 + F::cast_from(0.11853333333333333334e0_f64) * t77509 + F::cast_from(0.19755555555555555556e0_f64) * t88114 - F::cast_from(0.98777777777777777779e-1_f64) * t88118 - F::cast_from(0.35560000000000000001e0_f64) * t88122 + F::cast_from(0.35560000000000000001e0_f64) * t88126 - F::cast_from(0.29633333333333333334e-1_f64) * t88130 - F::cast_from(0.39511111111111111112e-1_f64) * t88134 + F::cast_from(0.61461728395061728396e-1_f64) * t51978;
    (t89144, t89157)
}
