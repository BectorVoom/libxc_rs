//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1407/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1407<F: Float>(t1340: F, t9318: F, t2491: F, t2514: F, t2495: F, t744: F) -> (F, F, F) {
    let t9320 = F::cast_from(0.35089341735807877242e1_f64) * t1340 * t9318;
    let t9321 = t2491 * t2514;
    let t9323 = t9321 * t2495 * t744;
    (t9320, t9321, t9323)
}
