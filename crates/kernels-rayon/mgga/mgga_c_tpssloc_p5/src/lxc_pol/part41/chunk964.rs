//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 964/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk964(t1339: f64, t2690: f64, t1336: f64, t1354: f64, t1307: f64, t3792: f64, t3788: f64, t835: f64, t1995: f64, t67: f64, t246: f64, t3777: f64, t3802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12366 = t12365 * t1354;
    let t12369 = t3792 * t1307;
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12429 = t3777 * t3802;
    (t12365, t12366, t12369, t12385, t12418, t12419, t12429)
}
