//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1203/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1203<F: Float>(t20018: F, t20055: F, t20093: F, t20126: F, t355: F, t377: F, t1175: F, t6689: F, t3464: F, t14781: F, t284: F, t5048: F, sigma0: F) -> (F, F, F) {
    let t20128 = t20018 + t20055 + t20093 + t20126;
    let t20129 = t20128 * t355;
    let t20130 = t20129 * sigma0;
    let t20131 = t20130 * t377;
    let t20133 = t1175 * t6689;
    let t20134 = t3464 * t20133;
    let t20136 = t14781 * t284;
    let t20137 = t20136 * t5048;
    (t20131, t20134, t20137)
}
