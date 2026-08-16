//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 390/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk390<F: Float>(t262: F, t830: F, t661: F, t655: F, t265: F, t321: F, t793: F, t27: F, t3814: F, t3810: F, t333: F, t797: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7581 = t262 * t830;
    let t7582 = t661 * t7581;
    let t7583 = F::cast_from(0.14784062966376104158e-3_f64) * t7582;
    let t7594 = t655 * t7581;
    let t7595 = F::cast_from(0.11111899192470324408e-1_f64) * t7594;
    let t7596 = t265 * t321;
    let t7597 = t793 * t7596;
    let t7599 = t3814 * t27;
    let t7603 = t3810 * t27;
    let t7617 = t265 * t333;
    let t7618 = t797 * t7617;
    (t7581, t7582, t7583, t7594, t7595, t7596, t7597, t7599, t7603, t7617, t7618)
}
