//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1276/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1276<F: Float>(t13148: F, t17708: F, t1209: F, t489: F, t3623: F, t370: F, t1214: F, t606: F, t3566: F, t13142: F, t13127: F, t3588: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t17709 = t13148 * t17708;
    let t17727 = t1209 * t489;
    let t17728 = t3623 * t370;
    let t17729 = t17727 * t17728;
    let t17730 = t606 * t1214;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    let t17747 = t13142 * t17708;
    let t17753 = t13127 * t17708;
    let t17784 = t3588 * t471;
    (t17709, t17729, t17730, t17736, t17747, t17753, t17784)
}
