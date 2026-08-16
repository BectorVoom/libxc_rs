//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1240/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1240(t4480: f64, t5728: f64, t4484: f64, t1705: f64, t4487: f64, t935: f64, t5570: f64, t6259: f64, t1232: f64, t1656: f64, t520: f64, t1265: f64, t1640: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19493 = t5728 * t4480;
    let t19495 = t5728 * t4484;
    let t19506 = t1705 * t4487;
    let t19507 = t19506 * t935;
    let t19509 = t6259 * t5570;
    let t19521 = t1656 * t1232 * t520;
    let t19535 = t1640 * t1265;
    (t19493, t19495, t19506, t19507, t19509, t19521, t19535)
}
