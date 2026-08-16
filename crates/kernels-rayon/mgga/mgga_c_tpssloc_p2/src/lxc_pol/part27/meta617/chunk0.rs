//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2095/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2095(t225: f64, t23410: f64, t6692: f64, t82632: f64, t6707: f64, t82573: f64, t6695: f64, t3166: f64, t6703: f64, t1049: f64, t6733: f64, t23366: f64, t23384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83276 = t23410 * t225;
    let t83281 = t82632 * t6692;
    let t83285 = t82573 * t6707;
    let t83287 = t82573 * t6695;
    let t83296 = t6703 * t3166;
    let t83303 = t6733 * t1049;
    let t83316 = t23384 * t23366;
    (t83276, t83281, t83285, t83287, t83296, t83303, t83316)
}
