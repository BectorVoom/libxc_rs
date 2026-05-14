//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 949/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk949<F: Float>(t2626: F, t4038: F, t2491: F, t745: F, t9368: F, t1340: F, t1330: F, t2608: F, t512: F, t169: F, t2552: F, t164: F, t2538: F, t729: F, t2556: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F) -> (F, F, F, F, F, F, F, F) {
    let t9422 = t4038 * t2626;
    let t9425 = t2491 * t9368 * t745;
    let t9427 = 0.35089341735807877242e1 * t1340 * t9425;
    let t9428 = t1330 * t2608;
    let t9429 = t512 * t9428;
    let t9432 = 1.0 / t2552 / t169;
    let t9433 = t164 * t9432;
    let t9434 = t2538 * t729;
    let t9435 = t9434 * t2556;
    let t9446 = -0.47063e1 * t9283 + 0.31375333333333333334e1 * t9286 - 0.36604555555555555556e1 * t9289 - 0.16068111111111111111e1 * t9292 + 0.28051666666666666666e0 * t9296 - 0.56103333333333333332e0 * t9298 - 0.6545388888888888889e0 * t9300 - 0.46308888888888888888e0 * t9303;
    (t9422, t9425, t9427, t9429, t9433, t9434, t9435, t9446)
}
