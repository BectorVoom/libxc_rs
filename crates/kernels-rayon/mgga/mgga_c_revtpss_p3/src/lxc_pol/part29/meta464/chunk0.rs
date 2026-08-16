//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1718/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1718(t212: f64, t7506: f64, t1358: f64, t689: f64, t2097: f64, t785: f64, t2439: f64, t2435: f64, t7493: f64, t26069: f64, t26277: f64, t26072: f64, t7515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26354 = t212 * t7506;
    let t26355 = t26354 * t1358;
    let t26356 = t689 * t26355;
    let t26358 = t785 * t2097;
    let t26359 = t26358 * t1358;
    let t26361 = 0.65049603595885220126e-3_f64 * t2439 * t26359;
    let t26363 = 0.73171657588172351096e-2_f64 * t2435 * t7493;
    let t26365 = 0.22849835011101738147e-2_f64 * t26069 * t26277;
    let t26366 = t26072 * t7515;
    (t26354, t26355, t26356, t26358, t26359, t26361, t26363, t26365, t26366)
}
