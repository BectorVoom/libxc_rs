//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 791/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk791(t10256: f64, t30204: f64, t6525: f64, t10272: f64, t2317: f64, t12830: f64, t1358: f64, t3129: f64, t31903: f64, t9074: f64, t10166: f64, t9086: f64) -> (f64, f64, f64, f64, f64) {
    let t42546 = t6525 * t30204 * t10256;
    let t42579 = t6525 * t10272 * t2317;
    let t42581 = t1358 * t12830;
    let t42587 = t9074 * t31903 * t3129;
    let t42590 = t9074 * t10166 * t9086;
    (t42546, t42579, t42581, t42587, t42590)
}
