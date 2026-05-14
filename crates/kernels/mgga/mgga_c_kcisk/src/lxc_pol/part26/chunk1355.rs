//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1355/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1355<F: Float>(t32266: F, t34866: F, t27095: F, t33652: F, t33643: F, t6357: F, t119753: F, t119755: F, t119758: F, t119760: F, t119762: F, t119764: F, t119766: F, t119768: F, t119771: F, t119773: F, t119775: F, t119777: F, t119779: F, t119781: F, t119783: F, t119785: F) -> (F, F, F, F) {
    let t119787 = t32266 * t34866;
    let t119789 = t33652 * t27095;
    let t119791 = t33643 * t6357;
    let t119793 = t119753 / 48.0 - t119755 / 12.0 - t119758 / 32.0 - t119760 / 24.0 - 2.0 / 9.0 * t119762 - t119764 / 24.0 + t119766 / 6.0 - t119768 / 128.0 - t119771 / 144.0 - t119773 / 96.0 + t119775 / 12.0 + t119777 / 4.0 - t119779 / 9.0 - t119781 / 24.0 + t119783 / 54.0 - t119785 / 96.0 - t119787 / 72.0 - t119789 / 32.0 - t119791 / 12.0;
    (t119787, t119789, t119791, t119793)
}
