//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 589/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk589<F: Float>(t3699: F, t5819: F, t1012: F, t1225: F, t5825: F, t3692: F, t344: F, t5843: F, t3618: F, t6421: F, t247: F, t1264: F, t6429: F, t6425: F, t1774: F, t1794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
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
    let t6679 = t247 * t6678;
    let t6682 = t1264 * t6425;
    let t6683 = t247 * t6682;
    let t6688 = t1774 * t1794;
    (t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6679, t6683, t6688)
}
