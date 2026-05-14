//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 726/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk726<F: Float>(t3391: F, t9422: F, t2300: F, t2982: F, t3387: F, t7927: F, t876: F, t3378: F, t3367: F, t3383: F, t3382: F, t2660: F, t9067: F, t8135: F, t1018: F, t2619: F) -> (F, F, F, F, F, F, F, F) {
    let t9423 = t3391 * t9422;
    let t9425 = t2982 * t2300;
    let t9426 = t3387 * t9425;
    let t9429 = t7927 * t876;
    let t9430 = t3378 * t9429;
    let t9432 = t3367 * t3383;
    let t9433 = t3382 * t9432;
    let t9435 = t2660 * t9067;
    let t9436 = t9435 * t8135;
    let t9438 = t2619 * t1018;
    (t9423, t9425, t9426, t9430, t9433, t9435, t9436, t9438)
}
