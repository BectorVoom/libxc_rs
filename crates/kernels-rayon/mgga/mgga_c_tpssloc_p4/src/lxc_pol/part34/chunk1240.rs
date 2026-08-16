//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1240/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1240(t28: f64, t265: f64, t504: f64, t108522: f64, t108574: f64, t108616: f64, t1409: f64, t20217: f64, t2071: f64, t29189: f64, t52: f64, t5398: f64, t7884: f64, t101150: f64, t102386: f64, t106755: f64, t106971: f64, t108533: f64, t113: f64, t1458: f64, t1459: f64, t1774: f64, t1983: f64, t20293: f64, t20347: f64, t2040: f64, t20563: f64, t20720: f64, t2075: f64, t2095: f64, t2096: f64, t24432: f64, t24995: f64, t26905: f64, t27188: f64, t28821: f64, t28826: f64, t28834: f64, t28943: f64, t28959: f64, t29197: f64, t29214: f64, t29252: f64, t4028: f64, t5460: f64, t652: f64, t67001: f64, t7042: f64, t74014: f64, t7685: f64, t7687: f64, t7943: f64, t9016: f64, t93966: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t108618 = piecewise3(t505, 0.0_f64, t108522);
    let t108628 = piecewise3(t401, t108574 + t108616, t108618 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t29189 * t1409 - 3.0_f64 / 2.0_f64 * t7884 * t5398 - t2071 * t20217 / 2.0_f64);
    let t108649 = -2.0_f64 * t7042 * t20720 - 2.0_f64 * t652 * t2075 * t20347 - 2.0_f64 * t67001 * t2040 - 6.0_f64 * t102386 * t1459 - 6.0_f64 * t4028 * t29214 - 6.0_f64 * t652 * t29197 * t1458 - 12.0_f64 * t27188 * t5460 - 3.0_f64 * t28821 * t7943 - t20293 * t2075 - 6.0_f64 * t28959 * t1774 + t106755 * t2096 + 9.0_f64 * t1983 * t26905 * t28834 - t113 * (t108533 + t108628) + 18.0_f64 * t24995 * t9016 * t20563 - 18.0_f64 * t24995 * t24432 * t106971 - 3.0_f64 * t28943 * t1774 + 18.0_f64 * t1983 * t93966 * t28826 + 9.0_f64 * t1983 * t101150 * t7687 + 18.0_f64 * t7685 * t29252 - t1983 * t2095 * t74014;
    t108649
}
