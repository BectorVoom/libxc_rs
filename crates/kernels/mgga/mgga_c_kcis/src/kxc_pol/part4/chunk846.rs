//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 846/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk846<F: Float>(t1482: F, t5526: F, t542: F, t3795: F, t3848: F, t5469: F, t5472: F, t5475: F, t5479: F, t469: F, t1315: F, t1893: F) -> (F, F, F, F, F) {
    let t5527 = t1482 * t5526;
    let t5528 = t542 * t5527;
    let t5538 = t3848 + F::new(0.5936111111111111111e-2) * t3795 + F::new(0.5936111111111111111e-2) * t5469 - F::new(0.11872222222222222222e-1) * t5472 + F::new(0.35616666666666666666e-1) * t5475 + F::new(0.35616666666666666666e-1) * t5479;
    let t5540 = F::new(0.62182e-1) * t5538 * t469;
    let t5541 = t1893 * t1315;
    (t5527, t5528, t5538, t5540, t5541)
}
