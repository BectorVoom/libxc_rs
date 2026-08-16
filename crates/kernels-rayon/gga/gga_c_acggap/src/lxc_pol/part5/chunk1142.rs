//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1142/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1142(t1008: f64, t5946: f64, t1032: f64, t6081: f64, t1856: f64, t3670: f64, t1036: f64, t1095: f64, t1795: f64, t398: f64, t864: f64, t1017: f64, t1131: f64, t12615: f64, t12621: f64, t12623: f64, t12626: f64, t12641: f64, t12646: f64, t1426: f64, t15529: f64, t1713: f64, t1772: f64, t3300: f64, t418: f64) -> f64 {
    let t20471 = t1008 * t5946;
    let t20478 = t1032 * t6081;
    let t20480 = t3670 * t1856;
    let t20490 = t1036 * t398 * t1095 * t1795 * t864;
    let t20495 = 0.48018900292238105409e-1_f64 * t12615 - 0.24009450146119052705e-1_f64 * t12621 + 0.24009450146119052705e-1_f64 * t12623 - t12626 - 0.17149607247227894789e-1_f64 * t20471 - 0.85748036236139473944e-2_f64 * t418 * t1426 * t1095 * t1713 * t1131 - 0.40015750243531754508e-2_f64 * t20478 - 0.45351183609335988442e-1_f64 * t20480 + 0.25724410870841842183e-2_f64 * t418 * t398 * t3300 * t1772 * t1017 - 0.85748036236139473944e-3_f64 * t20490 - 0.17149607247227894789e-2_f64 * t15529 - 455.0_f64 / 648.0_f64 * t12641 + 35.0_f64 / 432.0_f64 * t12646;
    t20495
}
