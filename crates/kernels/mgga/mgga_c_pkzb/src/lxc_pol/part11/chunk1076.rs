//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1076/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1076<F: Float>(t1953: F, t1975: F, t252: F, t17348: F, t2155: F, t5955: F, t655: F, t2003: F, t54: F, t300: F, t5633: F, t466: F, t779: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17637 = F::new(1.0) / t1975 / t1953;
    let t17638 = t252 * t17637;
    let t17664 = F::cast_from(0.17757530864197530864e0_f64) * t17348;
    let t17728 = F::cast_from(0.18467901234567901234e0_f64) * t17348;
    let t17752 = t2155 * t2155;
    let t17753 = F::new(1.0) / t17752;
    let t17787 = t5955 * t655;
    let t17848 = t54 * t2003;
    let t17852 = t300 * t5633;
    let t17867 = t466 * t779;
    (t17637, t17638, t17664, t17728, t17753, t17787, t17848, t17852, t17867)
}
