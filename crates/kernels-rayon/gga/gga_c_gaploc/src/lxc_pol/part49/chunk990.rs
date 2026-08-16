//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 990/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk990(t43007: f64, t7290: f64, t1841: f64, t7289: f64, t2508: f64, t3255: f64, t8637: f64, t2936: f64, t9689: f64, t13206: f64, t7137: f64, t3487: f64, t734: f64, t9636: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43008 = t7290 * t43007;
    let t43010 = t1841 * t7289 * t43008;
    let t43014 = 0.23071578690426672851e-1_f64 * t2508 * t8637 * t3255;
    let t43017 = 0.23071578690426672851e-1_f64 * t2508 * t2936 * t9689;
    let t43019 = 0.20508069947045931423e-1_f64 * t7137 * t13206;
    let t43023 = 0.85450291446024714263e-3_f64 * t1841 * t9636 * t3487 * t734;
    (t43008, t43010, t43014, t43017, t43019, t43023)
}
