//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1123/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1123(t22716: f64, t7697: f64, t7692: f64, t81186: f64, t1834: f64, t794: f64, t26197: f64, t80670: f64, t213: f64, t225: f64, t22724: f64, t26474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90503 = t22716 * t7697;
    let t90521 = t81186 * t7692;
    let t90544 = t794 * t1834;
    let t90551 = t80670 * t26197;
    let t90566 = t213 * t1834 * t225;
    let t90582 = t22724 * t26474;
    (t90503, t90521, t90544, t90551, t90566, t90582)
}
