//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1346/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1346(t24591: f64, t85639: f64, t1240: f64, t3242: f64, t1251: f64, t2244: f64, t24698: f64, t491: f64, t3247: f64, t2127: f64, t82631: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85640 = t85639 * t24591;
    let t85642 = t1240 * t3242;
    let t85643 = t2244 * t1251;
    let t85648 = t24698 * t491;
    let t85652 = t1240 * t3247;
    let t85660 = t2127 * t82631;
    (t85640, t85642, t85643, t85648, t85652, t85660)
}
