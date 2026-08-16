//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 474/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk474(t2089: f64, t911: f64, t1: f64, t7284: f64, t1422: f64, t6109: f64, t787: f64, t1984: f64, t7426: f64, t201: f64) -> (f64, f64, f64, f64, f64) {
    let t7428 = t911 * t2089;
    let t7442 = t7284 * t1;
    let t7512 = t6109 * t1422;
    let t7513 = t787 * t7512;
    let t7572 = t1984 * t7426;
    let t7573 = t201 * t2089;
    (t7428, t7442, t7513, t7572, t7573)
}
