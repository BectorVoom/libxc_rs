//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1049/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1049(t2639: f64, t3431: f64, t7284: f64, t787: f64, t13008: f64, t2087: f64, t4614: f64, t13133: f64, t2197: f64, t1445: f64, t43001: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t43941 = t787 * t7284 * t3431 * t2639;
    let t43944 = t2087 * t4614 * t13008;
    let t43946 = t2197 * t13133;
    let t43950 = t833 * t1445 * t43001;
    (t43941, t43944, t43946, t43950)
}
