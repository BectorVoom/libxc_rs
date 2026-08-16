//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1683/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1683(t4292: f64, t6646: f64, t1888: f64, t2647: f64, t4282: f64, t22986: f64, t6547: f64, t7529: f64, t25249: f64, t829: f64, t22996: f64, t4283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25284 = t6646 * t4292;
    let t25285 = t1888 * t25284;
    let t25287 = t4282 * t2647;
    let t25288 = t6646 * t25287;
    let t25289 = t22986 * t25288;
    let t25293 = t6547 * t7529;
    let t25299 = t25249 * t829;
    let t25300 = t6646 * t25299;
    let t25301 = t22986 * t25300;
    let t25303 = t22996 * t4283;
    (t25284, t25285, t25287, t25288, t25289, t25293, t25299, t25300, t25301, t25303)
}
