//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 241/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk241(t735: f64, t736: f64, t224: f64, t704: f64, t225: f64, t695: f64) -> (f64, f64, f64, f64) {
    let t738 = 0.54217906501508699211e-2_f64 * t735 * t736;
    let t739 = t704 * t224;
    let t740 = t225 * t695;
    let t741 = t739 * t740;
    (t738, t739, t740, t741)
}
