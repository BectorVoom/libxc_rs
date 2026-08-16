//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1259/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1259(t1881: f64, t3670: f64, t1896: f64, t3237: f64, t1181: f64, t13911: f64, t13915: f64, t13919: f64, t13923: f64, t13927: f64, t17798: f64, t17804: f64, t17811: f64, t4665: f64, t4680: f64, t4735: f64, t6337: f64, t6338: f64) -> f64 {
    let t23207 = t3670 * t1881;
    let t23209 = t3237 * t1896;
    let t23226 = 0.34013387707001991332e-1_f64 * t23207 - 0.40015750243531754508e-2_f64 * t23209 + 0.13719685797782315831e-1_f64 * t17798 + 0.51448821741683684367e-2_f64 * t4735 * t1181 * t6337 * t4665 + 0.68598428988911579156e-2_f64 * t17804 - 0.42874018118069736972e-3_f64 * t13911 + 0.10289764348336736873e-1_f64 * t4735 * t4680 * t6338 + 0.17149607247227894789e-2_f64 * t13915 - 0.51448821741683684367e-2_f64 * t13919 - 0.34299214494455789578e-2_f64 * t13923 - 0.68598428988911579156e-2_f64 * t13927 + 7.0_f64 / 72.0_f64 * t17811;
    t23226
}
