//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 914/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk914(t1097: f64, t25591: f64, t25605: f64, t25640: f64, t25651: f64, t27595: f64, t27599: f64, t27606: f64, t27609: f64, t27616: f64, t27621: f64, t27627: f64, t27631: f64, t27635: f64, t27640: f64, t27643: f64, t27647: f64, t4758: f64, t7144: f64, t7156: f64, t7162: f64, t7167: f64, t7170: f64, t7174: f64, t7825: f64, t7833: f64, t7837: f64) -> f64 {
    let t27650 = 0.17347256376410398924e1_f64 * t7144 * t27595 + 0.17347256376410398924e1_f64 * t25591 * t27599 - 0.4336814094102599731e0_f64 * t7825 * t7174 - 0.4336814094102599731e0_f64 * t7167 * t27606 + 0.8673628188205199462e0_f64 * t27609 * t7162 - 0.4336814094102599731e0_f64 * t7156 * t7837 + 0.13170898365871023197e1_f64 * t25651 * t4758 - 0.65854491829355115987e0_f64 * t27616 * t1097 - 0.4336814094102599731e0_f64 * t27621 * t7170 - 0.4336814094102599731e0_f64 * t25640 * t7833 - 0.4336814094102599731e0_f64 * t7167 * t27627 - 0.4336814094102599731e0_f64 * t7167 * t27631 + 0.17347256376410398924e1_f64 * t7144 * t27635 + 0.4336814094102599731e0_f64 * t27640 * t27643 + 0.8673628188205199462e0_f64 * t25605 * t27647;
    t27650
}
