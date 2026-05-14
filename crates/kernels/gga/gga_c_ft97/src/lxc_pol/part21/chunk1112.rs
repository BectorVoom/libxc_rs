//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1112/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1112<F: Float>(t6720: F, t8232: F, t27249: F, t8392: F, t26985: F, t38953: F, t6696: F, t1882: F, t27260: F, t6705: F, t2097: F, t5935: F, t6632: F, t26894: F, t27320: F, t27393: F, t376: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t107574 = t8232 * t6720;
    let t107589 = 2.0 / 27.0 * t8392 * t27249;
    let t107603 = 2.0 / 27.0 * t8392 * t26985;
    let t107614 = t38953 * t6696;
    let t107621 = 2.0 / 9.0 * t1882 * t27260;
    let t107625 = t8232 * t6705;
    let t107627 = t2097 * t5935;
    let t107650 = t8232 * t6632;
    let t107670 = 4.0 / 9.0 * t1882 * t26894;
    let t107680 = 2.0 / 9.0 * t1882 * t27320;
    let t107683 = 2.0 / 9.0 * t89 * t376 * t27393;
    (t107574, t107589, t107603, t107614, t107621, t107625, t107627, t107650, t107670, t107680, t107683)
}
