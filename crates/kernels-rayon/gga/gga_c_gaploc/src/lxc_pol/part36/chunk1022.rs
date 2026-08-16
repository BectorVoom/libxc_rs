//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1022/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1022(t10105: f64, t2969: f64, t11127: f64, t7324: f64, t3511: f64, t7822: f64, t13235: f64, t16710: f64, t841: f64, t11125: f64, t2592: f64, t2728: f64, t3459: f64, t5559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44194 = t2969 * t10105;
    let t44196 = t7324 * t11127;
    let t44198 = t7822 * t3511;
    let t44202 = 24.0_f64 * t16710 * t13235 * t841;
    let t44203 = t2592 * t11125;
    let t44207 = 12.0_f64 * t5559 * t3459 * t2728;
    (t44194, t44196, t44198, t44202, t44203, t44207)
}
