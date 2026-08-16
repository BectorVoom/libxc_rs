//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1168/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1168<F: Float>(t24739: F, t3068: F, t1244: F, t2132: F, t24683: F, t225: F, t460: F, t479: F, t23413: F, t3523: F, t7345: F, t3572: F, t7339: F) -> (F, F, F, F, F, F, F, F) {
    let t24740 = t24739 * t3068;
    let t24741 = t1244 * t24740;
    let t24744 = t2132 * t24683;
    let t24745 = t460 * t225;
    let t24746 = t24745 * t479;
    let t24747 = t24744 * t24746;
    let t24749 = t2132 * t23413;
    let t24752 = t7345 * t3523;
    let t24754 = t7339 * t3572;
    (t24740, t24741, t24745, t24746, t24747, t24749, t24752, t24754)
}
