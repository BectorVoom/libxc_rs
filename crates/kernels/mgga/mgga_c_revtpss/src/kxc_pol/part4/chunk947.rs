//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 947/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk947<F: Float>(t521: F, t9413: F, t182: F, t2490: F, t2495: F, t9368: F, t1340: F, t2626: F, t4038: F, t2491: F, t745: F, t1330: F, t2608: F) -> (F, F, F, F, F, F, F, F) {
    let t9415 = F::new(120.0) * t9413 * t521;
    let t9417 = F::new(1.0) / t2490 / t182;
    let t9419 = t9417 * t9368 * t2495;
    let t9421 = F::new(0.10389515463408878255e3) * t1340 * t9419;
    let t9422 = t4038 * t2626;
    let t9425 = t2491 * t9368 * t745;
    let t9427 = F::new(0.35089341735807877242e1) * t1340 * t9425;
    let t9428 = t1330 * t2608;
    (t9415, t9417, t9419, t9421, t9422, t9425, t9427, t9428)
}
