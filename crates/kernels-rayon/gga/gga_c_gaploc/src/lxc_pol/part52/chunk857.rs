//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 857/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk857(t15498: f64, t15499: f64, t44707: f64, t590: f64, t2679: f64, t3626: f64, t9800: f64, t43446: f64, t43454: f64, t2639: f64, t3614: f64, t7284: f64, t787: f64) -> (f64, f64, f64, f64, f64) {
    let t45277 = 0.61348681526273199482e1_f64 * t15498 * t15499 * t44707 * t590;
    let t45285 = t9800 * t3626 * t2679;
    let t45287 = 0.41708904943825497782e0_f64 * t43446;
    let t45288 = 0.35750489951850426669e0_f64 * t43454;
    let t45298 = 0.25025342966295298669e1_f64 * t787 * t7284 * t3614 * t2639;
    (t45277, t45285, t45287, t45288, t45298)
}
