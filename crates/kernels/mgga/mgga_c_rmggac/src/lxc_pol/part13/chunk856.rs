//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 856/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk856<F: Float>(t2115: F, t41028: F, t6444: F, t8708: F, t41055: F, t793: F, t2118: F, t41048: F, t2100: F, t41056: F, t2103: F, t41036: F, t39680: F, t4669: F, t27041: F, t38564: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41365 = t2115 * t41028;
    let t41371 = t6444 * t8708;
    let t41373 = t793 * t41055;
    let t41375 = t2118 * t41048;
    let t41377 = t2100 * t41056;
    let t41379 = t2103 * t41036;
    let t41381 = t2118 * t41036;
    let t41393 = t4669 * t39680;
    let t41395 = t27041 * t38564;
    (t41365, t41371, t41373, t41375, t41377, t41379, t41381, t41393, t41395)
}
