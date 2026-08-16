//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3738/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3738(t17448: f64, t17451: f64, t1121: f64, t6587: f64, t13148: f64, t70916: f64, t13142: f64, t12772: f64, t21218: f64, t3625: f64, t12784: f64, t12787: f64, t12855: f64, t12910: f64, t17429: f64, t17459: f64, t17713: f64, t17729: f64, t17730: f64, t17736: f64, t17750: f64, t20297: f64, t20838: f64, t21008: f64, t21119: f64, t21164: f64, t21257: f64, t3626: f64, t3720: f64, t5354: f64, t5407: f64, t57040: f64, t57571: f64, t58791: f64) -> f64 {
    let t71020 = t17448 * t17451;
    let t71029 = t6587 * t1121;
    let t71036 = t13148 * t70916;
    let t71039 = t13142 * t70916;
    let t71047 = t3625 * t12772 * t21218;
    let t71053 = 0.17149607247227894789e-2_f64 * t12910 * t3720 * t21164 * t17459 - 0.3811023832717309953e-3_f64 * t71020 + 0.57165357490759649296e-3_f64 * t58791 - 0.17149607247227894789e-2_f64 * t12855 * t3720 * t21257 * t21119 - 0.85748036236139473944e-3_f64 * t57040 * t5354 - 0.57165357490759649296e-3_f64 * t17736 * t3626 * t71029 * t17730 - 0.85748036236139473944e-3_f64 * t17429 * t20838 - 0.13719685797782315831e-1_f64 * t71036 * t17713 + 0.13719685797782315831e-1_f64 * t71039 * t17750 + 0.47637797908966374414e-3_f64 * t12784 * t21008 + 0.30488190661738479624e-2_f64 * t57571 * t5407 - 0.19055119163586549765e-3_f64 * t71047 - 0.28582678745379824648e-2_f64 * t17729 * t12787 * t20297 * t17730;
    t71053
}
