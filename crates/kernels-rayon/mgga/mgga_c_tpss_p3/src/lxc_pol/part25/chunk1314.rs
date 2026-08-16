//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1314/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1314(t43101: f64, t520: f64, t1265: f64, t5413: f64, t1640: f64, t4459: f64, t43602: f64, t5408: f64, t19809: f64, t63840: f64, t17930: f64, t52639: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69727 = t43101 * t520;
    let t69730 = t5413 * t1265;
    let t69734 = t1640 * t4459;
    let t69738 = t43602 * t520;
    let t69741 = t5408 * t1265;
    let t69789 = t63840 * t19809;
    let t69796 = t17930 * t52639;
    (t69727, t69730, t69734, t69738, t69741, t69789, t69796)
}
