//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 662/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk662<F: Float>(t517: F, t8340: F, t1007: F, t2933: F, t2948: F, t2951: F, t423: F, t1459: F, t2954: F, t3071: F, t5: F, t101: F, t3948: F, t4855: F, t2902: F, t3946: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8341 = t8340 * t517;
    let t8342 = t8341 * t1007;
    let t8344 = t2933 * t2948;
    let t8346 = t2951 * t423;
    let t8347 = t8346 * t1459;
    let t8348 = t8347 * t2954;
    let t8350 = t5 * t3071;
    let t8351 = t8350 * t101;
    let t8352 = t8351 * t1459;
    let t8353 = t3948 * t4855;
    let t8354 = t8352 * t8353;
    let t8356 = t2902 * t101;
    let t8357 = t8356 * t3946;
    (t8341, t8342, t8344, t8347, t8348, t8350, t8351, t8352, t8354, t8356, t8357)
}
