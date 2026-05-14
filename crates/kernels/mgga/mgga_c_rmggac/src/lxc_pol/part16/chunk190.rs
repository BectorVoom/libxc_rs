//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 190/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk190<F: Float>(t655: F, t656: F, t344: F, t36: F, t22: F, t349: F, t194: F, t202: F) -> (F, F, F, F, F) {
    let t657 = t655 * t656;
    let t659 = t344 * t36;
    let t661 = t349 * t22;
    let t662 = t661 * t656;
    let t671 = t194 * t202;
    (t657, t659, t661, t662, t671)
}
