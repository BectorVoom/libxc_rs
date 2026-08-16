//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1159/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1159(t22267: f64, t25997: f64, t22259: f64, t26024: f64, t6876: f64, t2018: f64, t22125: f64, t807: f64, t6864: f64, t94455: f64, t6846: f64, t22061: f64, t25986: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108566 = t25997 * t22267;
    let t108570 = t25997 * t22259;
    let t108576 = t26024 * t6876;
    let t108587 = t807 * t2018 * t22125;
    let t108590 = t94455 * t6864;
    let t108592 = t26024 * t6846;
    let t108601 = t2661 * t25986 * t22061;
    (t108566, t108570, t108576, t108587, t108590, t108592, t108601)
}
