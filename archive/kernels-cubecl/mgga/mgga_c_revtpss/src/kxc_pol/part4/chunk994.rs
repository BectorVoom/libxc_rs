//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 994/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk994<F: Float>(t10285: F, t27: F, t2231: F, t596: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t2240: F, t602: F, t2246: F, t599: F) -> (F, F, F, F, F, F) {
    let t10287 = F::cast_from(120.0_f64) * t10285 * t27;
    let t10288 = t2231 * t596;
    let t10290 = t592 * t2237;
    let t10292 = t2236 * t3;
    let t10293 = F::cast_from(1.0_f64) / t10292;
    let t10295 = F::cast_from(336.0_f64) * t25 * t10293;
    let t10298 = t2240 * t602;
    let t10301 = t599 * t2246;
    (t10287, t10288, t10290, t10295, t10298, t10301)
}
