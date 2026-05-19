//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 190/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk190<F: Float>(t655: F, t656: F, t344: F, t36: F, t22: F, t349: F, t653: F) -> (F, F) {
    let t657 = t655 * t656;
    let t659 = t344 * t36;
    let t661 = t349 * t22;
    let t662 = t661 * t656;
    let t664 = -F::cast_from(0.49892673757765869017e-2_f64) * t653 + F::cast_from(0.11364442355935559054e-2_f64) * t657 - F::cast_from(0.66380770525302906694e-4_f64) * t659 + F::cast_from(0.15120064397430106525e-4_f64) * t662;
    (t661, t664)
}
