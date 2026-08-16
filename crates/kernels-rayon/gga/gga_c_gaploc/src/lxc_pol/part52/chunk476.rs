//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 476/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk476(t2590: f64, t747: f64, t197: f64, t986: f64, t161: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t7822 = t2590 * t747;
    let t7887 = t197 * t986;
    let t7888 = t7887 * t161;
    let t7892 = t986 * t475;
    (t7822, t7887, t7888, t7892)
}
