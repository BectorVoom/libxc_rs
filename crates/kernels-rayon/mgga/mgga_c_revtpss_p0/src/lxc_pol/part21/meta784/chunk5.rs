//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2826/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2826(t11506: f64, t15542: f64, t4707: f64, t981: f64, t15538: f64, t3022: f64, t10356: f64, t15153: f64, t128: f64, t904: f64) -> (f64, f64, f64, f64) {
    let t51844 = 0.30762056574649219973e4_f64 * t981 * t11506 * t4707 * t15542;
    let t51846 = 0.70178683471615754484e1_f64 * t3022 * t15538;
    let t51847 = t15153 * t10356;
    let t51849 = t128 * t904 * t51847;
    (t51844, t51846, t51847, t51849)
}
