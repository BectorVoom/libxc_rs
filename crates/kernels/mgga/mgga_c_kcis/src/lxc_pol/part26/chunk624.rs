//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 624/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk624<F: Float>(t4134: F, t6922: F, t572: F, t571: F, t1494: F, t7202: F, t584: F, t6927: F, t583: F, t4286: F, t552: F, t7192: F, t577: F, t585: F, t1926: F, t488: F, t579: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7309 = t4134 * t6922;
    let t7310 = t572 * t7309;
    let t7311 = t571 * t7310;
    let t7313 = t1494 * t7202;
    let t7314 = t572 * t7313;
    let t7315 = t571 * t7314;
    let t7317 = t584 * t6927;
    let t7318 = t583 * t7317;
    let t7319 = t4286 * t7318;
    let t7321 = t7192 * t552;
    let t7322 = t7321 * t577;
    let t7323 = t7322 * t585;
    let t7327 = 1.0 / t488 / t579 / t1926;
    (t7309, t7310, t7311, t7313, t7314, t7315, t7318, t7319, t7321, t7322, t7323, t7327)
}
