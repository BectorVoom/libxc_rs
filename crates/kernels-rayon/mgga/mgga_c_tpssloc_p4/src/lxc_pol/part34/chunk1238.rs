//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1238/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1238(t25: f64, t265: f64, t394: f64, t108522: f64, t108096: f64, t108466: f64, t1409: f64, t20217: f64, t2064: f64, t29149: f64, t40: f64, t5398: f64, t7865: f64, t106618: f64, t106621: f64, t106636: f64, t106640: f64, t106647: f64, t106671: f64, t106686: f64, t106706: f64, t106712: f64, t108452: f64, t1877: f64, t20390: f64, t2057: f64, t24191: f64, t2522: f64, t26756: f64, t28: f64, t28771: f64, t28789: f64, t29106: f64, t4314: f64, t5966: f64, t7114: f64, t7649: f64, t7845: f64, t92319: f64, t93000: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t108523 = piecewise3(t395, 0.0_f64, t108522);
    let t108533 = piecewise3(t115, t108096 + t108466, t108523 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t29149 * t1409 + 3.0_f64 / 2.0_f64 * t7865 * t5398 + t2064 * t20217 / 2.0_f64);
    let t108574 = -9.0_f64 * t92319 * t28771 + t1877 * t108452 * t28 / 2.0_f64 + t1877 * t2057 * t20390 / 2.0_f64 - 9.0_f64 / 2.0_f64 * t24191 * t106621 + 3.0_f64 * t1877 * t93000 * t28789 - 3.0_f64 / 2.0_f64 * t1877 * t7114 * t106712 - t1877 * t7114 * t106636 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t1877 * t7845 * t5966 - 9.0_f64 * t24191 * t106706 - 3.0_f64 / 2.0_f64 * t1877 * t7114 * t106686 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t106647 + 3.0_f64 * t26756 * t106618 + 9.0_f64 * t4314 * t2057 * t106640 + 9.0_f64 / 2.0_f64 * t2522 * t29106 * t7649 + 9.0_f64 * t24191 * t106671;
    (t108533, t108574)
}
