//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1350/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1350(t1805: f64, t8275: f64, t1378: f64, t17993: f64, t18000: f64, t18006: f64, t18750: f64, t18767: f64, t18770: f64, t18789: f64, t18794: f64, t19736: f64, t19748: f64, t19767: f64, t20446: f64, t20475: f64, t20479: f64, t20482: f64, t20483: f64, t226: f64, t2407: f64, t3664: f64, t5571: f64, t5572: f64, t5577: f64, t5831: f64, t5843: f64, t61183: f64, t61222: f64, t61226: f64, t62671: f64, t6337: f64, t6343: f64, t6348: f64, t64034: f64, t64039: f64, t64042: f64, t64122: f64, t64135: f64, t64164: f64, t64168: f64, t64183: f64, t818: f64) -> f64 {
    let t66559 = t8275 * t1805;
    let t66601 = -4.0_f64 * t18006 * t62671 * t19748 - 2.0_f64 * t18006 * t18770 * t64122 - 4.0_f64 * t61222 * t20479 - 6.0_f64 * t19736 * t18767 + 4.0_f64 * t17993 * t20475 + 6.0_f64 * t19767 * t66559 * t64164 - 6.0_f64 * t19767 * t20482 * t64168 - 4.0_f64 * t64034 * t20483 + 6.0_f64 * t61226 * t18770 * t64042 + 2.0_f64 * t64135 * t5843 + 4.0_f64 * t5571 * t5572 * t20446 * t818 + t19767 * t18770 * t64039 - 2.0_f64 * t19767 * t20482 * t64183 + 2.0_f64 * t19736 * t18789 + t19736 * t18794 + t5571 * t5577 * t18750 * t1378 * t226 + 2.0_f64 * t5571 * t5577 * t5831 * t3664 * t226 + t61183 * t6348 + 2.0_f64 * t61183 * t6343 - 6.0_f64 * t5571 * t18000 * t6337 * t2407;
    t66601
}
