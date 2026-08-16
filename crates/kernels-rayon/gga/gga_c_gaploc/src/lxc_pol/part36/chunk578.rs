//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 578/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk578(t10122: f64, t447: f64, t1064: f64, t3340: f64, t535: f64, t3347: f64, t6305: f64, t7930: f64, t888: f64, t2268: f64, t2349: f64, t2765: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10123 = t10122 * t447;
    let t10124 = t1064 * t10123;
    let t10127 = t535 * t3340;
    let t10131 = 0.85365019907028448797e-1_f64 * t6305 * t3347;
    let t10132 = t7930 * t888;
    let t10134 = 0.85365019907028448797e-1_f64 * t2268 * t10132;
    let t10135 = t2765 * t2349;
    (t10123, t10124, t10127, t10131, t10134, t10135)
}
