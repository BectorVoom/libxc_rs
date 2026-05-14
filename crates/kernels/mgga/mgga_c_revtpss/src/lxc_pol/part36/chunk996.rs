//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 996/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk996<F: Float>(t1225: F, t22671: F, t1012: F, t13006: F, t22688: F, t13027: F, t13020: F, t1774: F, t6628: F, t3604: F, t3720: F, t3611: F, t24232: F, t247: F, t3618: F, t1264: F, t24248: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24816 = t1225 * t22671;
    let t24817 = t1012 * t24816;
    let t24820 = t13006 * t22688;
    let t24821 = t1012 * t24820;
    let t24826 = t13027 * t22688;
    let t24827 = t1012 * t24826;
    let t24830 = t13020 * t22688;
    let t24831 = t1012 * t24830;
    let t24834 = t1774 * t6628;
    let t24835 = t24834 * t3604;
    let t24836 = t3720 * t24835;
    let t24839 = t24834 * t3611;
    let t24840 = t3720 * t24839;
    let t24846 = t247 * t3618 * t24232;
    let t24858 = t247 * t1264 * t24248;
    (t24817, t24821, t24827, t24831, t24834, t24836, t24840, t24846, t24858)
}
