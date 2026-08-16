//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2097/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2097(t6695: f64, t82632: f64, t23317: f64, t23384: f64, t225: f64, t23572: f64, t23587: f64, t6698: f64, t3166: f64, t6688: f64, t23399: f64, t6692: f64, t82573: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83368 = t82632 * t6695;
    let t83398 = t23384 * t23317;
    let t83408 = t23572 * t225;
    let t83420 = t6698 * t23587;
    let t83424 = t6688 * t3166;
    let t83435 = t23384 * t23399;
    let t83441 = t82573 * t6692;
    (t83368, t83398, t83408, t83420, t83424, t83435, t83441)
}
