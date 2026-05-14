//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 227/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk227<F: Float>(t1038: F, t1041: F, t978: F, t431: F, t171: F, t388: F, t433: F, t151: F, t5: F, t959: F) -> (F, F, F, F, F) {
    let t1042 = t1038 * t978 * t1041;
    let t1044 = 0.17315859105681463759e2 * t431 * t1042;
    let t1045 = t388 * t171;
    let t1046 = t1045 * t433;
    let t1050 = 0.14764627977777777777e-2 * t5 * t959 * t151;
    (t1042, t1044, t1045, t1046, t1050)
}
