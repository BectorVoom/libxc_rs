//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 676/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk676(t12423: f64, t6485: f64, t882: f64, t3158: f64, t6470: f64, t874: f64, t9439: f64, t9438: f64, t587: f64, t9448: f64, t2487: f64, t6590: f64, t9291: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12424 = t6485 * t12423;
    let t12425 = t882 * t12424;
    let t12427 = t3158 * t6470;
    let t12428 = t882 * t12427;
    let t12444 = t9439 * t874;
    let t12445 = t9438 * t12444;
    let t12446 = t587 * t12445;
    let t12448 = t9448 * t874;
    let t12449 = t9438 * t12448;
    let t12450 = t2487 * t12449;
    let t12452 = t9291 * t6590;
    (t12424, t12425, t12427, t12428, t12444, t12445, t12446, t12448, t12449, t12450, t12452)
}
