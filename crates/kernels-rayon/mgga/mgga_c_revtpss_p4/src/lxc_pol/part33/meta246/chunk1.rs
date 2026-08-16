//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1093/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1093(t3699: f64, t5819: f64, t1012: f64, t1225: f64, t5825: f64, t3692: f64, t344: f64, t5843: f64, t3618: f64, t6421: f64, t247: f64, t1264: f64, t6429: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6652 = t3699 * t5819;
    let t6653 = t1012 * t6652;
    let t6658 = t1225 * t5825;
    let t6659 = t1012 * t6658;
    let t6662 = t3692 * t5819;
    let t6663 = t1012 * t6662;
    let t6667 = t5843 * t344;
    let t6672 = t3618 * t6421;
    let t6673 = t247 * t6672;
    let t6678 = t1264 * t6429;
    (t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6678)
}
