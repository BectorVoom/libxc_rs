//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 881/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk881(t2558: f64, t28152: f64, t9647: f64, t2554: f64, t7064: f64, t9637: f64, t12608: f64, t2549: f64, t12612: f64, t2562: f64, t28197: f64, t883: f64, t943: f64) -> (f64, f64, f64, f64, f64) {
    let t40696 = t9647 * t28152 * t2558;
    let t40699 = t7064 * t9637 * t2554;
    let t40744 = t2549 * t12608;
    let t40746 = t2549 * t12612;
    let t40750 = t943 * t2562 * t883 * t28197;
    (t40696, t40699, t40744, t40746, t40750)
}
