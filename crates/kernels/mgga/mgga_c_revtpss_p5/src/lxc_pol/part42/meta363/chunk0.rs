//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1180/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1180<F: Float>(t1261: F, t17720: F, t1209: F, t489: F, t3623: F, t370: F, t3566: F, t1121: F, t1774: F, t13142: F, t17708: F, t13127: F) -> (F, F, F, F, F, F) {
    let t17721 = t1261 * t17720;
    let t17727 = t1209 * t489;
    let t17728 = t3623 * t370;
    let t17729 = t17727 * t17728;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    let t17737 = t1774 * t1121;
    let t17747 = t13142 * t17708;
    let t17753 = t13127 * t17708;
    (t17721, t17729, t17736, t17737, t17747, t17753)
}
