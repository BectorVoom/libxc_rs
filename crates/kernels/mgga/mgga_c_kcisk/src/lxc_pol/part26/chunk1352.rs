//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1352/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1352<F: Float>(t27230: F, t33652: F, t119714: F, t119716: F, t119718: F, t119721: F, t119723: F, t119725: F, t119727: F, t119729: F, t119731: F, t119733: F, t119735: F, t119738: F, t119740: F, t119742: F, t119744: F, t119746: F, t119748: F) -> (F, F) {
    let t119750 = t33652 * t27230;
    let t119752 = t119714 / 144.0 - t119716 / 6.0 - t119718 / 8.0 - t119721 / 16.0 + t119723 / 96.0 + t119725 / 96.0 + t119727 / 6.0 - t119729 / 36.0 - t119731 / 288.0 - t119733 / 8.0 + t119735 / 24.0 + t119738 / 24.0 + t119740 / 128.0 - t119742 / 48.0 + t119744 / 48.0 + t119746 / 48.0 + t119748 / 288.0 - t119750 / 64.0;
    (t119750, t119752)
}
