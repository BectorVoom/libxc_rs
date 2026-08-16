//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 519/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk519<F: Float>(t1487: F, t4200: F, t1486: F, t469: F, t382: F, t41: F, t3742: F, t3783: F, t484: F, t3786: F, t470: F, t487: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4201 = t1487 * t4200;
    let t4203 = t1486 * t469;
    let t4204 = t41 * t382;
    let t4205 = t4204 * t3742;
    let t4206 = t4203 * t4205;
    let t4208 = t484 * t3783;
    let t4209 = t4208 * sigma0;
    let t4210 = t470 * t3786;
    let t4211 = t487 * t4210;
    (t4201, t4203, t4204, t4205, t4206, t4208, t4209, t4210, t4211)
}
