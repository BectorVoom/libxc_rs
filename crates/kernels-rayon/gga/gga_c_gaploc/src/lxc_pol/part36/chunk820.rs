//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 820/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk820(t12996: f64, t18067: f64, t2365: f64, t31586: f64, t4391: f64, t31591: f64, t12993: f64, t7014: f64, t10215: f64, t123: f64, t883: f64, t2487: f64, t2488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41623 = t18067 * t12996;
    let t41624 = 0.59584149919750711116e-1_f64 * t41623;
    let t41626 = t4391 * t2365 * t31586;
    let t41627 = 0.59584149919750711116e-1_f64 * t41626;
    let t41629 = t4391 * t2365 * t31591;
    let t41630 = 0.59584149919750711116e-1_f64 * t41629;
    let t41631 = t7014 * t12993;
    let t41634 = t10215 * t123 * t883;
    let t41636 = t2487 * t2488 * t41634;
    (t41624, t41627, t41630, t41631, t41634, t41636)
}
