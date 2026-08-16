//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 796/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk796(t1020: f64, t17611: f64, t135: f64, t5889: f64, t973: f64, t5893: f64, t5884: f64, t248: f64, t3101: f64, t5878: f64, t3039: f64, t3051: f64, t5685: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17612 = t1020 * t17611;
    let t17615 = t135 * t5889;
    let t17616 = t973 * t17615;
    let t17620 = t135 * t5893;
    let t17621 = t973 * t17620;
    let t17624 = t135 * t5884;
    let t17625 = t973 * t17624;
    let t17655 = t248 * t3101 * t5878;
    let t17656 = t3039 * t17655;
    let t17659 = t248 * t3051 * t5685;
    (t17612, t17616, t17621, t17625, t17656, t17659)
}
