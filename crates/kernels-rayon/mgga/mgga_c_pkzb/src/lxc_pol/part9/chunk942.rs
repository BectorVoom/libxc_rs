//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 942/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk942(t1979: f64, t2848: f64, t721: f64, t1971: f64, t2852: f64, t1938: f64, t1977: f64, t2801: f64, t2820: f64, t5825: f64, t5897: f64, t7268: f64, t7271: f64, t7274: f64, t7277: f64, t7281: f64, t7284: f64, t7288: f64, t7293: f64, t7296: f64) -> (f64, f64, f64, f64) {
    let t7299 = t2848 * t1979;
    let t7300 = t7299 * t721;
    let t7303 = t2852 * t1971;
    let t7306 = -t7268 + t7271 + t7274 + t7277 - t7281 - t7284 - t7288 - 4.0_f64 * t5897 * t2801 + 0.64327917994770140268e2_f64 * t5825 * t2820 + 6.0_f64 * t1938 * t7293 + 0.35089341735807877242e1_f64 * t1977 * t7296 + 0.34631718211362927518e2_f64 * t1977 * t7300 + 0.17315859105681463759e2_f64 * t1977 * t7303;
    (t7299, t7300, t7303, t7306)
}
