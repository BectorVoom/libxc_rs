//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 507/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk507<F: Float>(t684: F, t127: F, t129: F, t130: F, t2379: F, t2496: F, t2500: F, t2507: F, t2508: F, t2514: F, t2518: F, t60: F, t756: F, t763: F, t764: F, t768: F) -> (F,) {
    let t2522 = t684 * t684;
    let t2526 = -0.43802864444444444443e-3 * t127 * t2496 * t130 - 0.2e-22 * t763 * t2500 * t130 - 0.26281718666666666666e-2 * t127 * t756 * t768 + 0.19711288999999999999e-2 * t2507 * t2508 + 0.19711288999999999999e-2 * t763 * t764 * t768 + 0.39422577999999999998e-2 * t127 * t129 * t2514 - 0.19711288999999999999e-2 * t127 * t129 * t2518 - 4.0 * t2522 - 4.0 * t60 * t2379;
    (t2526,)
}
