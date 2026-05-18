//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 941/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk941<F: Float>(t2514: F, t2596: F, t746: F, t1340: F, t2491: F, t2495: F, t744: F, t215: F, t681: F, t268: F, t702: F) -> (F, F, F, F, F) {
    let t9318 = t2596 * t2514 * t746;
    let t9320 = F::new(0.35089341735807877242e1) * t1340 * t9318;
    let t9321 = t2491 * t2514;
    let t9323 = t9321 * t2495 * t744;
    let t9325 = F::new(0.51947577317044391277e2) * t1340 * t9323;
    let t9326 = t215 * t681;
    let t9329 = F::new(0.71233333333333333332e-1) * t268 * t9326 * t702;
    (t9318, t9320, t9323, t9325, t9329)
}
