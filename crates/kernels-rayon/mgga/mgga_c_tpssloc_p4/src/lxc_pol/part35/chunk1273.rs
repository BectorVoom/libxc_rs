//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1273/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1273(t50: f64, t9300: f64, t11588: f64, t2127: f64, t221: f64, t1240: f64, t3242: f64, t3247: f64, t82631: f64) -> (f64, f64, f64, f64, f64) {
    let t85539 = t50 * t9300;
    let t85639 = t2127 * t221 * t11588;
    let t85642 = t1240 * t3242;
    let t85652 = t1240 * t3247;
    let t85660 = t2127 * t82631;
    (t85539, t85639, t85642, t85652, t85660)
}
