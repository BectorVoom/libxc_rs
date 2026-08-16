//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 672/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk672(t12399: f64, t866: f64, t1233: f64, t157: f64, t874: f64, t9439: f64, t9438: f64, t587: f64, t9448: f64, t2487: f64, t12381: f64, t286: f64, t708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12400 = t12399 * t866;
    let t12411 = 1.0_f64 / t1233;
    let t12412 = t157 * t12411;
    let t12444 = t9439 * t874;
    let t12445 = t9438 * t12444;
    let t12446 = t587 * t12445;
    let t12448 = t9448 * t874;
    let t12449 = t9438 * t12448;
    let t12450 = t2487 * t12449;
    let t12555 = t12381 * t286 * t708;
    (t12400, t12411, t12412, t12444, t12445, t12446, t12448, t12449, t12450, t12555)
}
