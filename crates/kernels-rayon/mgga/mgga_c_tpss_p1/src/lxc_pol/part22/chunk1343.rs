//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1343/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1343(t198: f64, t205: f64, t5848: f64, t206: f64, t6353: f64, t2161: f64, t6337: f64, t768: f64, t10764: f64, t10837: f64, t17991: f64, t17993: f64, t18000: f64, t18006: f64, t18009: f64, t1805: f64, t18753: f64, t18770: f64, t18771: f64, t18797: f64, t19736: f64, t19762: f64, t19767: f64, t19781: f64, t20463: f64, t20474: f64, t20482: f64, t20494: f64, t20506: f64, t226: f64, t2425: f64, t3699: f64, t5568: f64, t5571: f64, t5577: f64, t5834: f64, t62671: f64, t6342: f64, t6351: f64, t64034: f64, t64060: f64, t64063: f64, t64118: f64, t64190: f64, t64204: f64, t818: f64) -> (f64, f64, f64, f64) {
    let t66311 = t198 * t205 * t5848;
    let t66317 = t198 * t206 * t6353;
    let t66328 = t6337 * t2161;
    let t66362 = t768 * t6337;
    let t66379 = t5571 * t5577 * t66328 * t226 - 6.0_f64 * t5571 * t18000 * t6342 * t2425 + 4.0_f64 * t18753 * t3699 - 2.0_f64 * t18006 * t18770 * t64190 - 4.0_f64 * t64060 * t18771 + 4.0_f64 * t18006 * t20482 * t64063 + t19736 * t18797 + 2.0_f64 * t19767 * t62671 * t19781 - 12.0_f64 * t5571 * t18000 * t20474 * t818 + 2.0_f64 * t64034 * t20494 - 4.0_f64 * t18006 * t62671 * t19762 - 4.0_f64 * t18006 * t18770 * t64118 - 4.0_f64 * t18006 * t66362 * t18009 + t19767 * t18770 * t64204 + t5571 * t5577 * t1805 * t10764 * t226 - t17991 * t6351 + 4.0_f64 * t5834 * t10837 - 12.0_f64 * t17993 * t20463 - 2.0_f64 * t5568 * t20506;
    (t66311, t66317, t66328, t66379)
}
