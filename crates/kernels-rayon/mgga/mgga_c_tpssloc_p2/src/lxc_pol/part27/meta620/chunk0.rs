//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2098/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2098(t1920: f64, t2966: f64, t6699: f64, t1921: f64, t82457: f64, t23314: f64, t23384: f64, t6707: f64, t82632: f64, t23734: f64, t3216: f64, t11094: f64, t6818: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83444 = t1920 * t2966 * t6699;
    let t83453 = t1921 * t82457;
    let t83457 = t23384 * t23314;
    let t83459 = t82632 * t6707;
    let t83468 = t23734 * t3216;
    let t83472 = t6818 * t11094;
    (t83444, t83453, t83457, t83459, t83468, t83472)
}
