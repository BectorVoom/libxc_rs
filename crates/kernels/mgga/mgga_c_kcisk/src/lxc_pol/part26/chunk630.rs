//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 630/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk630<F: Float>(t6272: F, t6307: F, t467: F, t488: F, t4204: F, t5996: F, t4203: F, t469: F, t485: F, t1284: F, t41: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t6308 = t6272 + t6307;
    let t6309 = t6308 * t467;
    let t6310 = t6309 * sigma0;
    let t6311 = t6310 * t488;
    let t6313 = t4204 * t5996;
    let t6314 = t4203 * t6313;
    let t6316 = t485 * t469;
    let t6317 = t41 * t1284;
    (t6308, t6309, t6310, t6311, t6313, t6314, t6316, t6317)
}
