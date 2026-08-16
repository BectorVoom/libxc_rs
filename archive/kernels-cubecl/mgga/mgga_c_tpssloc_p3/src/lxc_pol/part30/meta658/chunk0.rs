//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2077/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2077<F: Float>(t90524: F, t22892: F, t7691: F, t80645: F, t26206: F, t6883: F, t1834: F, t794: F, t6891: F, t22704: F, t26355: F, t81326: F) -> (F, F, F, F, F, F) {
    let t90525 = F::cast_from(0.16449340668482264365e-1_f64) * t90524;
    let t90533 = t22892 * t80645 * t7691;
    let t90534 = F::cast_from(0.16449340668482264365e-1_f64) * t90533;
    let t90541 = t6883 * t26206;
    let t90542 = F::cast_from(0.38381794893125283518e-1_f64) * t90541;
    let t90544 = t794 * t1834;
    let t90546 = t22892 * t90544 * t6891;
    let t90547 = F::cast_from(0.16449340668482264365e-1_f64) * t90546;
    let t90549 = t22704 * t81326 * t26355;
    (t90525, t90534, t90542, t90544, t90547, t90549)
}
