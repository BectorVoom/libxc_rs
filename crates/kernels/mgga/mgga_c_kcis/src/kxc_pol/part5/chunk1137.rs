//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1137/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1137<F: Float>(t5541: F, t5574: F, t11543: F, t6954: F, t3856: F, t6986: F, t653: F, t6938: F) -> (F, F, F, F) {
    let t21176 = 2.0 * t5541 * t5574;
    let t21178 = 2.0 * t11543 * t6954;
    let t21180 = 1.0 * t3856 * t6986;
    let t21186 = t653 * t6938;
    (t21176, t21178, t21180, t21186)
}
