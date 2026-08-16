//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1209/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1209(t16721: f64, t16775: f64, t16779: f64, t16783: f64, t16787: f64, t16886: f64, t16889: f64, t16893: f64, t16897: f64, t29127: f64, t29128: f64, t1020: f64, t2706: f64) -> (f64, f64) {
    let t29707 = t29127 - t16886 - t16889 - t29128 - t16893 + t16897 + t16721 - t16775 - t16779 + t16783 - t16787;
    let t29713 = t1020 * t2706;
    (t29707, t29713)
}
