//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1269/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1269(t10018: f64, t159: f64, t1658: f64, t2146: f64, t2147: f64, t2222: f64, t2385: f64, t2394: f64, t2400: f64, t33250: f64, t33566: f64, t38361: f64, t38370: f64, t38377: f64, t38379: f64, t38382: f64, t40601: f64, t40709: f64, t42181: f64, t616: f64, t619: f64, t639: f64, t6438: f64, t7912: f64, t7931: f64, t8004: f64, t8306: f64, t9003: f64, t9136: f64) -> f64 {
    let t42189 = 0.17347256376410398924e1_f64 * t2146 * t2147 * t2385 * t1658 - 0.39512695097613069591e1_f64 * t2222 * t6438 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t2394 * t1658 + 0.13170898365871023197e1_f64 * t38361 - 0.13170898365871023197e1_f64 * t33250 - 0.4336814094102599731e0_f64 * t40601 * t639 + 0.8673628188205199462e0_f64 * t7912 * t10018 + 0.8673628188205199462e0_f64 * t9003 * t9136 + 0.8673628188205199462e0_f64 * t33566 * t2400 + t38370 - 0.4336814094102599731e0_f64 * t616 * t619 * t159 * t42181 + t38377 - t38379 - 0.8673628188205199462e0_f64 * t7931 * t8306 * t40709 - t38382;
    t42189
}
