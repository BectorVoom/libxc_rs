//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1076/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1076(t1953: f64, t1975: f64, t252: f64, t17348: f64, t2155: f64, t5955: f64, t655: f64, t2003: f64, t54: f64, t300: f64, t5633: f64, t466: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17637 = 1.0_f64 / t1975 / t1953;
    let t17638 = t252 * t17637;
    let t17664 = 0.17757530864197530864e0_f64 * t17348;
    let t17728 = 0.18467901234567901234e0_f64 * t17348;
    let t17752 = t2155 * t2155;
    let t17753 = 1.0_f64 / t17752;
    let t17787 = t5955 * t655;
    let t17848 = t54 * t2003;
    let t17852 = t300 * t5633;
    let t17867 = t466 * t779;
    (t17637, t17638, t17664, t17728, t17753, t17787, t17848, t17852, t17867)
}
