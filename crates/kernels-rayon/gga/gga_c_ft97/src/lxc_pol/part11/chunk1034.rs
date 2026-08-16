//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1034/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1034(t13468: f64, t13474: f64, t17958: f64, t224: f64, t2378: f64, t2387: f64, t2388: f64, t2417: f64, t2426: f64, t2427: f64, t3761: f64, t3789: f64, t41497: f64, t41542: f64, t41548: f64, t41549: f64, t41557: f64, t41561: f64, t41569: f64, t41573: f64, t41577: f64, t678: f64, t680: f64, t695: f64, t709: f64, t807: f64, t9524: f64, t9601: f64, t9609: f64, t9617: f64, t9677: f64) -> f64 {
    let t41588 = -t224 * t695 * (t41497 + t41542) + 24.0_f64 * t224 * t41548 * t41549 - 0.23238868087529279928e-2_f64 * t13468 * t2378 * t2417 * t2388 - 0.279058811357253504e-1_f64 * t13474 * t41557 * t9617 - 0.279058811357253504e0_f64 * t17958 * t3761 * t41561 + 8.0_f64 * t3789 * t2426 * t9677 * t709 + 6.0_f64 * t224 * t2427 * t41569 + 0.1116235245429014016e-1_f64 * t2387 * t9609 * t41573 - 0.19352371901929178119e-4_f64 * t678 * t807 * t41577 - 0.69716604262587839785e-3_f64 * t678 * t9524 * t41577 + 0.46509801892875584e-1_f64 * t2387 * t680 * t9601 * t709;
    t41588
}
