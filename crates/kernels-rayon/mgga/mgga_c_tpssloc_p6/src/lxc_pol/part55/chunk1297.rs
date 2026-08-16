//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1297/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1297(t120687: f64, t120691: f64, t120697: f64, t120699: f64, t120702: f64, t120708: f64, t120719: f64, t120721: f64, t120728: f64, t120730: f64, t120735: f64, t123228: f64, t123229: f64, t123235: f64, t123244: f64, t125910: f64, t1459: f64, t32609: f64, t4026: f64, t4037: f64, t4073: f64, t8913: f64) -> f64 {
    let t125963 = -2.0_f64 * t125910 * t1459 - 2.0_f64 * t32609 * t4037 - 2.0_f64 * t32609 * t4073 - t4026 * t8913 - t120687 - t120691 + t120697 + t120699 + t120702 - t120708 - t120719 - t120721 - t120728 - t120730 - t120735 - 2.0_f64 * t123228 - 6.0_f64 * t123229 + 2.0_f64 * t123235 - 4.0_f64 * t123244;
    t125963
}
