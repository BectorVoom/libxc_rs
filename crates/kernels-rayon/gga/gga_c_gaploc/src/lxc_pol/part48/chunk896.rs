//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 896/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk896(t1445: f64, t2087: f64, t37200: f64, t935: f64, t11016: f64, t3651: f64, t15498: f64, t15499: f64, t44707: f64, t590: f64, t2679: f64, t3626: f64, t9800: f64) -> (f64, f64, f64, f64) {
    let t45264 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t37200 * t935;
    let t45269 = 0.16683561977530199113e1_f64 * t3651 * t11016;
    let t45277 = 0.61348681526273199482e1_f64 * t15498 * t15499 * t44707 * t590;
    let t45285 = t9800 * t3626 * t2679;
    (t45264, t45269, t45277, t45285)
}
