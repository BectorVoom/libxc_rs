//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3757/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3757(t1222: f64, t17471: f64, t20298: f64, t20302: f64, t1260: f64, t57465: f64, t21334: f64, t1042: f64, t1261: f64, t1266: f64, t12832: f64, t17265: f64, t17347: f64, t21143: f64, t21166: f64, t21275: f64, t3600: f64, t3604: f64, t3640: f64, t3644: f64, t5302: f64, t5312: f64, t59159: f64, t65433: f64, t68324: f64, t68355: f64, t70343: f64) -> f64 {
    let t71571 = t1222 * t17471 * t20298;
    let t71582 = t1222 * t17471 * t20302;
    let t71585 = t57465 * t1260;
    let t71590 = t21334 * t1260;
    let t71597 = 0.47637797908966374414e-3_f64 * t1261 * t1042 * t5302 * t65433 + 0.85748036236139473944e-3_f64 * t3600 * t1042 * t70343 * t3604 + t71571 / 54.0_f64 + t1222 * t5312 * t68355 / 108.0_f64 + t1222 * t5312 * t68324 / 36.0_f64 - 0.85748036236139473944e-3_f64 * t12832 * t21166 + t71582 / 162.0_f64 + 0.17149607247227894789e-2_f64 * t59159 - 0.25724410870841842183e-2_f64 * t71585 * t17347 + 0.85748036236139473944e-3_f64 * t21275 * t17265 - 0.28582678745379824648e-3_f64 * t71590 * t1266 - 0.14291339372689912324e-3_f64 * t21143 * t3640 - 0.28582678745379824648e-3_f64 * t21143 * t3644;
    t71597
}
