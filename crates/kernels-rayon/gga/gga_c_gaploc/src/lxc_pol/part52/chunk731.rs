//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 731/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk731(t177: f64, t208: f64, t4347: f64, t1397: f64, t4390: f64, t1238: f64, t4072: f64, t4081: f64, t92: f64, t153: f64, t155: f64, t4080: f64) -> (f64, f64, f64, f64, f64) {
    let t17293 = t177 / t4347 / t208;
    let t18067 = t1397 * t4390;
    let t18089 = 1.0_f64 / t4072 / t1238;
    let t18091 = t18089 * t92 * t4081;
    let t18096 = t153 / t4080 / t155;
    (t17293, t18067, t18089, t18091, t18096)
}
