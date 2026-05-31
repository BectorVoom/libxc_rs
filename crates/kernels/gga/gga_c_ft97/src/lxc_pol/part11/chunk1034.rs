//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1034/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1034<F: Float>(t13468: F, t13474: F, t17958: F, t224: F, t2378: F, t2387: F, t2388: F, t2417: F, t2426: F, t2427: F, t3761: F, t3789: F, t41497: F, t41542: F, t41548: F, t41549: F, t41557: F, t41561: F, t41569: F, t41573: F, t41577: F, t678: F, t680: F, t695: F, t709: F, t807: F, t9524: F, t9601: F, t9609: F, t9617: F, t9677: F) -> F {
    let t41588 = -t224 * t695 * (t41497 + t41542) + F::cast_from(24.0_f64) * t224 * t41548 * t41549 - F::cast_from(0.23238868087529279928e-2_f64) * t13468 * t2378 * t2417 * t2388 - F::cast_from(0.279058811357253504e-1_f64) * t13474 * t41557 * t9617 - F::cast_from(0.279058811357253504e0_f64) * t17958 * t3761 * t41561 + F::cast_from(8.0_f64) * t3789 * t2426 * t9677 * t709 + F::cast_from(6.0_f64) * t224 * t2427 * t41569 + F::cast_from(0.1116235245429014016e-1_f64) * t2387 * t9609 * t41573 - F::cast_from(0.19352371901929178119e-4_f64) * t678 * t807 * t41577 - F::cast_from(0.69716604262587839785e-3_f64) * t678 * t9524 * t41577 + F::cast_from(0.46509801892875584e-1_f64) * t2387 * t680 * t9601 * t709;
    t41588
}
