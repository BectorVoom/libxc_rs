//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1176/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1176(t34550: f64, t34553: f64, t34555: f64, t34557: f64, t34560: f64, t34563: f64, t34565: f64, t34567: f64, t34570: f64, t34573: f64, t34576: f64, t3144: f64, t34372: f64) -> (f64, f64) {
    let t34578 = 0.45289771048911752714e-7_f64 * t34550 + 0.67530371184977617164e-6_f64 * t34553 + 0.67530371184977617164e-6_f64 * t34555 + 0.33765185592488808582e-6_f64 * t34557 + 0.52838066223730378166e-7_f64 * t34560 - 0.58366874983904959946e-8_f64 * t34563 - 0.6629778687778673199e-7_f64 * t34565 - 0.33148893438893365995e-7_f64 * t34567 + 0.687148483626368822e-6_f64 * t34570 - 0.33765185592488808582e-6_f64 * t34573 - 0.45020247456651744776e-7_f64 * t34576;
    let t34582 = t34372 * t3144;
    (t34578, t34582)
}
