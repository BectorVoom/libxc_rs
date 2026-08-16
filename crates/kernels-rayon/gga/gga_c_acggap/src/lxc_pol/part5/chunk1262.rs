//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1262/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1262(t3409: f64, t5873: f64, t1106: f64, t1181: f64, t1884: f64, t4282: f64, t12457: f64, t1817: f64, t1089: f64, t1095: f64, t1173: f64, t13889: f64, t13929: f64, t13934: f64, t13936: f64, t13939: f64, t13943: f64, t1749: f64, t17895: f64, t17902: f64, t372: f64, t418: f64, t5506: f64) -> f64 {
    let t23274 = t3409 * t5873;
    let t23285 = t4282 * t1181 * t1884 * t1106;
    let t23288 = t12457 * t1817;
    let t23295 = 0.20007875121765877254e-2_f64 * t23274 - 35.0_f64 / 108.0_f64 * t17895 + 0.80031500487063509016e-2_f64 * t13929 - 0.40015750243531754508e-2_f64 * t13934 + 0.40015750243531754508e-2_f64 * t13936 - t13939 + t13943 + 0.34299214494455789578e-2_f64 * t1173 * t13889 * t1749 - 0.17149607247227894789e-1_f64 * t23285 + 455.0_f64 / 324.0_f64 * t17902 + 0.11337795902333997111e-1_f64 * t23288 + 0.34299214494455789578e-2_f64 * t418 * t1089 * t1095 * t5506 * t372;
    t23295
}
