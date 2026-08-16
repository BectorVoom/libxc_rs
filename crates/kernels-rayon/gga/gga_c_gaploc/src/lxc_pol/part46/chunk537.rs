//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 537/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk537(t2549: f64, t3248: f64, t7221: f64, t883: f64, t2562: f64, t943: f64, t3240: f64, t2558: f64, t2717: f64, t2537: f64, t2554: f64, t7064: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9622 = 0.64087718584518535698e-3_f64 * t2549 * t3248;
    let t9624 = t883 * t7221;
    let t9625 = t2562 * t9624;
    let t9627 = 0.64087718584518535698e-3_f64 * t943 * t9625;
    let t9629 = 0.64087718584518535698e-3_f64 * t2549 * t3240;
    let t9630 = t2717 * t2558;
    let t9632 = 0.64087718584518535698e-3_f64 * t943 * t9630;
    let t9633 = t2537 * t2554;
    let t9635 = 0.64087718584518535698e-3_f64 * t7064 * t9633;
    (t9622, t9624, t9627, t9629, t9632, t9635)
}
