//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 962/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk962<F: Float>(t180: F, t26654: F, t7671: F, t838: F, t2209: F, t2802: F, t233: F, t7684: F, t911: F, t7827: F, t915: F, t7673: F, t7676: F, t7679: F, t380: F, t982: F) -> (F, F, F, F, F, F, F, F) {
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26658 = t2802 * t2209;
    let t26659 = t233 * t26658;
    let t26660 = t26659 / 16.0;
    let t26662 = t911 * t7684;
    let t26663 = t26662 / 8.0;
    let t26664 = t915 * t7827;
    let t26665 = t233 * t26664;
    let t26666 = t26665 / 8.0;
    let t26667 = t7673 * t7676;
    let t26668 = t26667 / 8.0;
    let t26669 = t7673 * t7679;
    let t26670 = t26669 / 8.0;
    let t26671 = t380 * t982;
    (t26655, t26656, t26660, t26663, t26666, t26668, t26670, t26671)
}
