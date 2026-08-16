//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1219/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1219(t18788: f64, t5577: f64, t1805: f64, t226: f64, t2364: f64, t18782: f64, t1708: f64, t18750: f64, t228: f64, t1707: f64, t17991: f64, t17993: f64, t18006: f64, t1809: f64, t18751: f64, t18753: f64, t18767: f64, t18771: f64, t18775: f64, t18779: f64, t18784: f64, t2408: f64, t2426: f64, t253: f64, t5568: f64, t5571: f64, t5834: f64, t5838: f64, t5843: f64, t5846: f64, t819: f64) -> (f64, f64, f64, f64, f64) {
    let t18789 = t5577 * t18788;
    let t18794 = t5577 * t1805 * t2364 * t226;
    let t18797 = t5577 * t18782 * t226;
    let t18800 = t1708 * t228 * t18750;
    let t18802 = -t1707 * t18800 - t17991 * t1809 + 4.0_f64 * t17993 * t5838 + 2.0_f64 * t17993 * t5843 - 4.0_f64 * t18006 * t18771 + t18751 * t253 - 2.0_f64 * t18753 * t819 - 6.0_f64 * t18767 * t5571 + 4.0_f64 * t18775 * t5571 + 2.0_f64 * t18779 * t5571 - 2.0_f64 * t18784 * t5571 + 2.0_f64 * t18789 * t5571 + t18794 * t5571 + t18797 * t5571 + 2.0_f64 * t2408 * t5834 - t2426 * t5834 - 2.0_f64 * t5568 * t5846;
    (t18789, t18794, t18797, t18800, t18802)
}
