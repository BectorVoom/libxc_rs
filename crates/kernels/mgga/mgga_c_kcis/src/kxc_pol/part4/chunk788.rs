//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 788/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk788<F: Float>(t1477: F, t5481: F, t542: F, t1098: F, t1996: F, t1961: F, t531: F, t833: F, t3766: F, t518: F, t1319: F, t3786: F, t509: F, t543: F, t1419: F, t1962: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5482 = t1477 * t5481;
    let t5483 = t542 * t5482;
    let t5486 = t1098 * t1996;
    let t5488 = t1961 * t531;
    let t5489 = t5488 * t833;
    let t5490 = t3766 * t5489;
    let t5493 = t518 * t1961;
    let t5494 = t5493 * t1319;
    let t5495 = t3786 * t5494;
    let t5498 = t509 * t543;
    let t5499 = t1962 * t1419;
    (t5482, t5483, t5486, t5488, t5490, t5493, t5494, t5495, t5498, t5499)
}
