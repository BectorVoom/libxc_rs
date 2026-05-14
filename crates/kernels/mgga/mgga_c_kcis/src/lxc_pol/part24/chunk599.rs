//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 599/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk599<F: Float>(t3437: F, t6720: F, t388: F, t6613: F, t387: F, t1187: F, t3346: F, t6496: F, t382: F, t358: F, t6555: F, t6683: F, t6685: F, t6687: F, t6691: F, t6694: F, t6698: F, t6702: F, t6706: F, t6710: F, t6712: F, t6714: F, t6718: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6721 = t3437 * t6720;
    let t6723 = t388 * t6613;
    let t6724 = t387 * t6723;
    let t6725 = t1187 * t6724;
    let t6727 = t3346 * t6496;
    let t6728 = t387 * t6727;
    let t6729 = t382 * t6728;
    let t6731 = t358 * t6555;
    let t6732 = t387 * t6731;
    let t6733 = t382 * t6732;
    let t6735 = t6683 / 16.0 - t6685 / 8.0 + t6687 / 12.0 + t6691 / 8.0 - t6694 / 12.0 - t6698 / 16.0 - t6702 / 72.0 + t6706 / 24.0 - t6710 / 256.0 + t6712 / 128.0 - t6714 / 96.0 - t6718 / 128.0 + t6721 / 96.0 + t6725 / 256.0 - t6729 / 576.0 - t6733 / 192.0;
    (t6721, t6723, t6724, t6725, t6727, t6728, t6729, t6731, t6732, t6733, t6735)
}
