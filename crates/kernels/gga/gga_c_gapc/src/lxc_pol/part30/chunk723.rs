//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 723/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk723<F: Float>(t1022: F, t9356: F, t1928: F, t3096: F, t3094: F, t5541: F, t612: F, t1671: F, t5544: F, t2712: F, t3430: F, t1044: F, t640: F, t916: F, t128: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t9357 = t1022 * t9356;
    let t9359 = t3096 * t1928;
    let t9360 = t3094 * t9359;
    let t9362 = t5541 * t612;
    let t9363 = t1671 * t5544;
    let t9364 = t9362 * t9363;
    let t9383 = t3096 * t2712;
    let t9384 = t3430 * t9383;
    let t9386 = t640 * t1044;
    let t9387 = t916 * t9386;
    let t9388 = t6 * t128;
    (t9357, t9360, t9364, t9384, t9386, t9387, t9388)
}
