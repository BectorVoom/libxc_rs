//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 800/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk800(t12608: f64, t731: f64, t12612: f64, t12623: f64, t12629: f64, t2549: f64, t948: f64, t9796: f64, t9829: f64, t1967: f64, t28236: f64, t7810: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40896 = t731 * t12608;
    let t40898 = t731 * t12612;
    let t40900 = t731 * t12623;
    let t40902 = t2549 * t12629;
    let t40942 = t9796 * t948 * t9829;
    let t40946 = t7810 * t1967 * t883 * t28236;
    (t40896, t40898, t40900, t40902, t40942, t40946)
}
