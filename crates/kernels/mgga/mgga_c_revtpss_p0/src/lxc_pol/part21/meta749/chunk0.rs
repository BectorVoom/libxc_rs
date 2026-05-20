//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2625/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2625<F: Float>(t1857: F, t9855: F, t9410: F, t9413: F, t47081: F, t5571: F, t9372: F, t13597: F, t2496: F, t123: F, t2630: F, t5566: F) -> (F, F, F, F, F, F, F) {
    let t48290 = t9855 * t1857;
    let t48291 = F::new(144.0) * t48290;
    let t48292 = t9410 * t1857;
    let t48293 = F::new(240.0) * t48292;
    let t48294 = t9413 * t1857;
    let t48295 = F::new(120.0) * t48294;
    let t48296 = F::new(4.0) * t47081;
    let t48297 = t5571 * t9372;
    let t48298 = F::cast_from(0.10254018858216406658e4_f64) * t48297;
    let t48299 = t13597 * t2496;
    let t48300 = F::cast_from(0.51947577317044391276e2_f64) * t48299;
    let t48302 = t5566 * t123 * t2630;
    (t48291, t48293, t48295, t48296, t48298, t48300, t48302)
}
