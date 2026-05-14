//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 442/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk442<F: Float>(t1804: F, t376: F, t375: F, t1747: F, t355: F, t381: F) -> (F, F, F, F) {
    let t1805 = t376 * t1804;
    let t1806 = t375 * t1805;
    let t1808 = t1747 * t355;
    let t1809 = t1808 * t381;
    (t1805, t1806, t1808, t1809)
}
