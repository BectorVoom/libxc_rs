//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1953/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1953(t5: f64, t101805: f64, t101824: f64, t101849: f64, t101875: f64, t101896: f64, t101919: f64, t101949: f64, t101975: f64, t117: f64, t7535: f64, t9593: f64, t101767: f64, t1310: f64, t13425: f64, t13532: f64, t13540: f64, t13544: f64, t1843: f64, t2056: f64, t2089: f64, t2322: f64, t26154: f64, t26399: f64, t26676: f64, t27123: f64, t28196: f64, t28198: f64, t28652: f64, t28658: f64, t28696: f64, t4246: f64, t4248: f64, t4254: f64, t4293: f64, t508: f64, t5517: f64, t651: f64, t7359: f64, t7367: f64, t7373: f64, t7474: f64, t98484: f64, t98487: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t101979 = piecewise3(t8, 0.0_f64, t101805 + t101824 + t101849 + t101875 + t101896 + t101919 + t101949 + t101975);
    let t101980 = t101979 * t117;
    let t102005 = t7535 * t9593;
    let t102009 = -2.0_f64 * t101767 * t508 - t13425 * t2089 - 2.0_f64 * t4246 * t7474 - 4.0_f64 * t7359 * t13540 - 4.0_f64 * t7359 * t13532 - 2.0_f64 * t26676 * t1843 - t101980 * t508 - 2.0_f64 * t28652 * t1310 - 2.0_f64 * t4248 * t26154 - 2.0_f64 * t98484 * t2056 - 4.0_f64 * t98487 * t2056 - 4.0_f64 * t27123 * t7367 - 4.0_f64 * t2322 * t28696 - 4.0_f64 * t4254 * t28696 - 4.0_f64 * t651 * t5517 * t7373 - 4.0_f64 * t26399 * t4293 - 4.0_f64 * t28658 * t4293 - 2.0_f64 * t7359 * t13544 + 4.0_f64 * t28196 * t102005 * t28198;
    (t101980, t102009)
}
