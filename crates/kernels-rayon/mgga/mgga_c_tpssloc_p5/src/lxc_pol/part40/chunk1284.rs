//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1284/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1284(t1851: f64, t671: f64, t1441: f64, t4072: f64, t19534: f64, t88: f64, t1458: f64, t4025: f64, t5493: f64, t649: f64, t5464: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75795 = t1851 * t671;
    let t96356 = t1441 * t4072;
    let t96657 = t88 * t19534;
    let t96683 = t4025 * t1458;
    let t96709 = t649 * t5493;
    let t96715 = t5464 * t666;
    (t75795, t96356, t96657, t96683, t96709, t96715)
}
