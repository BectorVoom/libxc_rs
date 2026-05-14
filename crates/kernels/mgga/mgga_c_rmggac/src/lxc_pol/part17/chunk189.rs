//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 189/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk189<F: Float>(t27: F, t649: F, t648: F, t305: F, t36: F, t22: F, t326: F, t262: F) -> (F, F, F, F, F) {
    let t650 = t27 * t649;
    let t651 = t648 * t650;
    let t653 = t305 * t36;
    let t655 = t326 * t22;
    let t656 = t262 * t36;
    (t650, t651, t653, t655, t656)
}
