//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1162/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1162<F: Float>(t5490: F, t709: F, t1953: F, t1975: F, t252: F, t1847: F, t1898: F, t17348: F, t2155: F, t2027: F, t5728: F, t2009: F, t5955: F, t1843: F, t2030: F, t2003: F, t54: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17633 = t709 * t5490;
    let t17637 = 1.0 / t1975 / t1953;
    let t17638 = t252 * t17637;
    let t17655 = t1847 * t1898;
    let t17664 = 0.17757530864197530864e0 * t17348;
    let t17728 = 0.18467901234567901234e0 * t17348;
    let t17752 = t2155 * t2155;
    let t17753 = 1.0 / t17752;
    let t17765 = t2027 * t5728;
    let t17766 = t5955 * t2009;
    let t17782 = t2030 * t1843;
    let t17848 = t54 * t2003;
    (t17633, t17637, t17638, t17655, t17664, t17728, t17753, t17765, t17766, t17782, t17848)
}
