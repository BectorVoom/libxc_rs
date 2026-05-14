//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1078/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1078<F: Float>(t20018: F, t20055: F, t20093: F, t20126: F, t355: F, t377: F, t1175: F, t6689: F, t3464: F, t14781: F, t284: F, t5048: F, t1797: F, t5185: F, t19112: F, t359: F, sigma0: F) -> (F, F, F, F, F) {
    let t20128 = t20018 + t20055 + t20093 + t20126;
    let t20129 = t20128 * t355;
    let t20130 = t20129 * sigma0;
    let t20131 = t20130 * t377;
    let t20133 = t1175 * t6689;
    let t20134 = t3464 * t20133;
    let t20136 = t14781 * t284;
    let t20137 = t20136 * t5048;
    let t20139 = t1797 * t5185;
    let t20141 = t359 * t19112;
    (t20131, t20134, t20137, t20139, t20141)
}
