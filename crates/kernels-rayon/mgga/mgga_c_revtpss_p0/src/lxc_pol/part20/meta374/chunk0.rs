//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1356/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1356(t2723: f64, t40262: f64, t10666: f64, t221: f64, t2484: f64, t2485: f64, t2482: f64, t2719: f64, t596: f64, t10852: f64, t2645: f64, t10858: f64, t10863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40325 = t2723 * t2723;
    let t40326 = t40262 * t40325;
    let t40333 = t2484 * t2485 * t221 * t10666;
    let t40336 = t2482 * t2719 * t596;
    let t40337 = t40336 * t10852;
    let t40339 = t2645 * t2645;
    let t40340 = t40339 * t2723;
    let t40345 = t10858 * t10863;
    (t40326, t40333, t40337, t40339, t40340, t40345)
}
