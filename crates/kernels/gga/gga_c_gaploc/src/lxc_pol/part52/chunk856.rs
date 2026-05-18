//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 856/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk856<F: Float>(t2617: F, t3630: F, t7803: F, t11894: F, t1445: F, t2087: F, t2530: F, t11801: F, t41105: F, t37200: F, t935: F, t11016: F, t3651: F) -> (F, F, F, F, F) {
    let t45246 = t7803 * t3630 * t2617;
    let t45247 = F::new(0.19171462976960374838e0) * t45246;
    let t45251 = F::new(0.69017266717057349418e1) * t2087 * t1445 * t11894 * t2530;
    let t45256 = F::new(0.42900587942220512003e1) * t11801 * t41105;
    let t45264 = F::new(0.69017266717057349418e1) * t2087 * t1445 * t37200 * t935;
    let t45269 = F::new(0.16683561977530199113e1) * t3651 * t11016;
    (t45247, t45251, t45256, t45264, t45269)
}
