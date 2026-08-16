//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 856/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk856(t2617: f64, t3630: f64, t7803: f64, t11894: f64, t1445: f64, t2087: f64, t2530: f64, t11801: f64, t41105: f64, t37200: f64, t935: f64, t11016: f64, t3651: f64) -> (f64, f64, f64, f64, f64) {
    let t45246 = t7803 * t3630 * t2617;
    let t45247 = 0.19171462976960374838e0_f64 * t45246;
    let t45251 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t11894 * t2530;
    let t45256 = 0.42900587942220512003e1_f64 * t11801 * t41105;
    let t45264 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t37200 * t935;
    let t45269 = 0.16683561977530199113e1_f64 * t3651 * t11016;
    (t45247, t45251, t45256, t45264, t45269)
}
