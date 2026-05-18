//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 942/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk942<F: Float>(t9160: F, t158: F, t157: F, t2491: F, t812: F, t2593: F, t2585: F, t2484: F, t2618: F, t2526: F, t808: F, t137: F, t8998: F) -> (F, F, F, F, F, F) {
    let t9161 = F::new(1.0) / t9160;
    let t9162 = t158 * t9161;
    let t9163 = t157 * t9162;
    let t9165 = t812 * t2491;
    let t9166 = t2593 * t9165;
    let t9168 = t2585 * t812;
    let t9170 = t2484 * t2618;
    let t9172 = t812 * t2526;
    let t9173 = t808 * t9172;
    let t9175 = t8998 * t137;
    (t9163, t9166, t9168, t9170, t9173, t9175)
}
