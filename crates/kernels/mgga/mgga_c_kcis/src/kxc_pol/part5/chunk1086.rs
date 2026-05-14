//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1086/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1086<F: Float>(t19582: F, t19586: F, t19591: F, t19596: F, t19601: F, t19603: F, t19607: F, t19612: F, t19617: F, t19622: F, t19628: F, t19633: F, t19636: F, t19640: F, t19642: F, t19645: F, t19648: F, t19651: F, t19658: F) -> (F,) {
    let t20289 = -0.51588271604938271605e-2 * t19582 + 0.11607361111111111111e-2 * t19586 + 0.51588271604938271604e-3 * t19591 + 0.38691203703703703703e-3 * t19596 - 0.11607361111111111111e-2 * t19601 + 0.77382407407407407407e-3 * t19603 - 0.30952962962962962963e-2 * t19607 - 0.10446625e-1 * t19612 + 0.23214722222222222221e-2 * t19617 + 0.69644166666666666664e-2 * t19622 - 0.23214722222222222222e-2 * t19628 - 0.23214722222222222222e-2 * t19633 + 0.11607361111111111111e-2 * t19636 + 0.23214722222222222221e-2 * t19640 - 0.23214722222222222221e-2 * t19642 + 0.92858888888888888885e-2 * t19645 + 0.23214722222222222222e-2 * t19648 + 0.11607361111111111111e-2 * t19651 + 0.11607361111111111111e-2 * t19658;
    (t20289,)
}
