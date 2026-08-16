//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1257/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1257(t5975: f64, t997: f64, t6205: f64, t6211: f64, t1016: f64, t1083: f64, t1181: f64, t13861: f64, t1460: f64, t1487: f64, t1524: f64, t17729: f64, t17733: f64, t17740: f64, t3201: f64, t398: f64, t418: f64, t4735: f64, t4762: f64, t4838: f64, t4875: f64, t506: f64, t513: f64, t5819: f64, t6337: f64) -> f64 {
    let t23154 = t997 * t5975;
    let t23156 = t997 * t6205;
    let t23158 = t997 * t6211;
    let t23172 = -0.34299214494455789578e-2_f64 * t418 * t398 * t3201 * t5819 + 0.34299214494455789578e-2_f64 * t17729 - 0.17149607247227894789e-2_f64 * t418 * t398 * t1083 * t4875 * t513 - 0.34299214494455789578e-2_f64 * t418 * t398 * t1083 * t1487 * t1524 - 0.17149607247227894789e-2_f64 * t418 * t398 * t1083 * t506 * t4838 - 0.40015750243531754508e-1_f64 * t23154 + 0.16006300097412701803e-1_f64 * t23156 - 0.16006300097412701803e-1_f64 * t23158 + 0.10289764348336736873e-1_f64 * t4735 * t1181 * t1016 * t1487 * t1460 + 0.12004725073059526352e-1_f64 * t13861 + 0.10289764348336736873e-1_f64 * t4735 * t1181 * t6337 * t4762 - 0.64025200389650807212e-1_f64 * t17733 + 0.34299214494455789578e-2_f64 * t17740;
    t23172
}
