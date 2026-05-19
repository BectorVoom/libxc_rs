//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 194/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk194<F: Float>(t118: F, t665: F, t646: F, t651: F) -> F {
    let t666 = t118 * t665;
    let t668 = F::cast_from(0.14967802127329760705e-1_f64) * t646 - F::cast_from(0.34093327067806677161e-2_f64) * t651 + F::cast_from(0.19957069503106347607e-1_f64) * t666;
    t668
}
