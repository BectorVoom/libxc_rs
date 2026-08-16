//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 100/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk100(t155: f64, t153: f64, t135: f64, t145: f64, t455: f64, t458: f64, t456: f64, t459: f64, t134: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t462 = t155 * t155;
    let t463 = 1.0_f64 / t462;
    let t464 = t153 * t463;
    let t465 = t464 * t135;
    let t467 = t455 * t145 * t458;
    let t470 = -7.0_f64 / 128.0_f64 * t456 * t145 * t459 + 7.0_f64 / 384.0_f64 * t465 * t467;
    let t471 = f64::ln(t134);
    (t462, t463, t464, t467, t470, t471)
}
