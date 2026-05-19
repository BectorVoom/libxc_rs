//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 835/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk835<F: Float>(t41027: F, t793: F, t41035: F, t797: F, t41055: F, t851: F, t854: F, t3810: F, t40920: F, t41031: F, t25529: F, t36: F) -> (F, F, F, F, F, F, F, F) {
    let t41191 = t793 * t41027;
    let t41195 = t797 * t41035;
    let t41230 = t851 * t41055;
    let t41231 = F::cast_from(0.17701538806747441785e-2_f64) * t41230;
    let t41233 = t854 * t41035;
    let t41234 = F::cast_from(0.21241846568096930142e-2_f64) * t41233;
    let t41241 = t3810 * t40920;
    let t41242 = F::cast_from(0.14869292597667851099e-1_f64) * t41241;
    let t41247 = t854 * t41031;
    let t41257 = t797 * t41031;
    let t41262 = t25529 * t36;
    (t41191, t41195, t41231, t41234, t41242, t41247, t41257, t41262)
}
