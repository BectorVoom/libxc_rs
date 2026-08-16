//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3700/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3700(t17608: f64, t5292: f64, t17547: f64, t5265: f64, t1261: f64, t20906: f64, t3172: f64, t17416: f64, t5391: f64, t21272: f64, t3636: f64, t1042: f64, t1252: f64, t1260: f64, t17550: f64, t44264: f64, t44270: f64, t44273: f64, t44276: f64, t5268: f64, t5384: f64, t5386: f64, t56246: f64, t59241: f64, t65829: f64, t65947: f64, t69875: f64) -> f64 {
    let t70088 = t17608 * t5292;
    let t70091 = t17547 * t5265;
    let t70102 = t1261 * t3172 * t20906;
    let t70112 = t5391 * t17416;
    let t70114 = t21272 * t3636;
    let t70119 = -0.45732285992607719436e-2_f64 * t70088 * t1252 - 0.30488190661738479624e-2_f64 * t70091 + 0.14291339372689912324e-2_f64 * t1261 * t1042 * t17550 * t65829 + 0.85748036236139473944e-2_f64 * t1261 * t1042 * t56246 * t65947 - 0.3811023832717309953e-3_f64 * t70102 - 0.57165357490759649296e-3_f64 * t5384 * t1042 * t5268 * t69875 + 0.1270341277572436651e-3_f64 * t44264 - 0.95275595817932748826e-4_f64 * t44270 - 0.47637797908966374413e-4_f64 * t44273 + 0.47637797908966374413e-4_f64 * t44276 - 0.33875767401931644027e-3_f64 * t70112 - 0.6436395806367012365e-2_f64 * t70114 + 0.17149607247227894789e-2_f64 * t59241 * t1260 * t5386;
    t70119
}
