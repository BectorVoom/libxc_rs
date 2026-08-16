//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3694/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3694(t1260: f64, t20850: f64, t11262: f64, t3600: f64, t6630: f64, t17225: f64, t5391: f64, t21183: f64, t3172: f64, t3711: f64, t20875: f64, t1042: f64, t1261: f64, t12809: f64, t16771: f64, t17199: f64, t17204: f64, t17344: f64, t17412: f64, t17550: f64, t1789: f64, t21028: f64, t21257: f64, t3584: f64, t3714: f64, t3720: f64, t5270: f64, t5296: f64, t57136: f64, t5825: f64, t65370: f64, t65947: f64) -> f64 {
    let t69906 = t20850 * t1260;
    let t69910 = t3600 * t11262 * t6630;
    let t69916 = t5391 * t17225;
    let t69936 = t3711 * t3172 * t21183;
    let t69939 = t3711 * t3172 * t20875;
    let t69943 = -0.76220476654346199062e-2_f64 * t1261 * t1042 * t57136 * t65947 + 0.28582678745379824648e-3_f64 * t69906 * t3714 - 0.95275595817932748827e-4_f64 * t69910 - 0.51448821741683684367e-2_f64 * t17344 * t1042 * t1789 * t16771 + 0.40650920882317972832e-2_f64 * t69916 + 0.30488190661738479624e-2_f64 * t5391 * t17199 + 0.91464571985215438872e-2_f64 * t5391 * t17204 + 0.14291339372689912324e-3_f64 * t3711 * t1042 * t5296 * t5825 * t3584 + 0.85748036236139473944e-3_f64 * t12809 * t3720 * t21257 * t21028 + 0.14291339372689912324e-2_f64 * t1261 * t1042 * t17550 * t65370 + 0.19055119163586549765e-3_f64 * t69936 + 0.3811023832717309953e-3_f64 * t69939 + 0.60976381323476959248e-2_f64 * t17412 * t5270;
    t69943
}
