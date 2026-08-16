//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 452/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk452(t3885: f64, t547: f64, t821: f64, t820: f64, t98: f64, t316: f64, t815: f64, t1579: f64, t825: f64, t101: f64, t814: f64, t154: f64, t1583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4879 = t3885 * t547 * t821;
    let t4882 = t98 * t820;
    let t4883 = t815 * t316;
    let t4886 = t1579 * t825;
    let t4889 = t101 * t814;
    let t4892 = t1583 * t154;
    (t4879, t4882, t4883, t4886, t4889, t4892)
}
