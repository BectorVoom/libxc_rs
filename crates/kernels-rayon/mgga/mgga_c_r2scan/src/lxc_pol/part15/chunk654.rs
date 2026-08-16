//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 654/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk654(t1053: f64, t1102: f64, t3692: f64, t3432: f64, t3442: f64, t3445: f64, t3451: f64, t3577: f64, t3581: f64, t3585: f64, t3621: f64, t3624: f64, t3690: f64) -> f64 {
    let t3694 = t1102 * t1053 * t3692;
    let t3696 = -t3432 + t3442 - t3445 - t3451 - 0.36021158228745895953e-3_f64 * t3690 + 0.15243824895787514157e-3_f64 * t3694 - t3577 - t3581 + t3585 - t3621 + t3624;
    t3696
}
