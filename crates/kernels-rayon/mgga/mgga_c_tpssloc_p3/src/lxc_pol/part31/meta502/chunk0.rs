//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1698/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1698(t19660: f64, t550: f64, t6976: f64, t1992: f64, t19743: f64, t6330: f64, t6890: f64, t6889: f64, t22685: f64, t26193: f64, t7700: f64, t1985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28167 = t19660 * t550;
    let t28168 = t6976 * t28167;
    let t28169 = t1992 * t28168;
    let t28181 = t19743 * t550;
    let t28182 = t6976 * t28181;
    let t28183 = t1992 * t28182;
    let t28191 = t6890 * t6330;
    let t28192 = t6889 * t28191;
    let t28193 = t22685 * t28192;
    let t28195 = t26193 * t7700;
    let t28196 = t1985 * t28195;
    (t28167, t28168, t28169, t28181, t28182, t28183, t28191, t28192, t28193, t28195, t28196)
}
