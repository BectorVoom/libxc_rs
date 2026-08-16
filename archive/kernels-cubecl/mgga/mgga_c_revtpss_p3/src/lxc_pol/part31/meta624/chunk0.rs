//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2075/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2075<F: Float>(t93190: F, t99211: F, t25374: F, t98848: F, t25378: F, t99403: F, t25375: F, t99161: F, t1580: F, t25338: F, t689: F, t25365: F, t27279: F) -> (F, F, F, F, F, F, F, F) {
    let t99460 = t93190 * t99211;
    let t99463 = t98848 * t25374;
    let t99465 = F::cast_from(0.51405703062096148812e-1_f64) * t99463 * t25378;
    let t99466 = t99403 * t25374;
    let t99468 = F::cast_from(0.28912093960683998208e-1_f64) * t99466 * t25378;
    let t99472 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t99161;
    let t99475 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t25338 * t1580;
    let t99480 = F::cast_from(0.25702851531048074406e-1_f64) * t25365 * t27279;
    (t99460, t99463, t99465, t99466, t99468, t99472, t99475, t99480)
}
