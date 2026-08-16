//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 880/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk880(t2562: f64, t35558: f64, t883: f64, t943: f64, t13542: f64, t2549: f64, t11613: f64, t1897: f64, t7675: f64, t2508: f64, t33760: f64, t9014: f64) -> (f64, f64, f64, f64) {
    let t45028 = t943 * t2562 * t883 * t35558;
    let t45029 = 0.32043859292259267849e-3_f64 * t45028;
    let t45030 = t2549 * t13542;
    let t45031 = 0.32043859292259267849e-3_f64 * t45030;
    let t45034 = 0.92286314761706691403e-1_f64 * t1897 * t11613 * t7675;
    let t45037 = 0.18457262952341338281e0_f64 * t2508 * t9014 * t33760;
    (t45029, t45031, t45034, t45037)
}
