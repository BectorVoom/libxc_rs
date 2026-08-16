//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 824/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk824(t2558: f64, t28438: f64, t10036: f64, t1980: f64, t10928: f64, t6574: f64, t822: f64, t123: f64, t15499: f64, t27997: f64, t7290: f64, t28013: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28439 = t28438 * t2558;
    let t28594 = t1980 * t10036;
    let t28640 = t822 * t10928 * t6574;
    let t28641 = t15499 * t123;
    let t28648 = t7290 * t27997;
    let t28652 = t7290 * t28013;
    (t28439, t28594, t28640, t28641, t28648, t28652)
}
