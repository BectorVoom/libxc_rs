//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1310/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1310<F: Float>(t10285: F, t27: F, t2231: F, t596: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t2240: F, t602: F, t2246: F, t599: F) -> (F, F, F, F, F, F) {
    let t10287 = F::new(120.0) * t10285 * t27;
    let t10288 = t2231 * t596;
    let t10290 = t592 * t2237;
    let t10292 = t2236 * t3;
    let t10293 = F::new(1.0) / t10292;
    let t10295 = F::new(336.0) * t25 * t10293;
    let t10298 = t2240 * t602;
    let t10301 = t599 * t2246;
    (t10287, t10288, t10290, t10295, t10298, t10301)
}
