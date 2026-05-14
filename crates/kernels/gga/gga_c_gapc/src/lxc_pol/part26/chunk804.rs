//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 804/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk804<F: Float>(t190: F, t2207: F, t10346: F, t442: F, t875: F, t3439: F, t6939: F, t19: F, t786: F, t147: F, t3296: F, t2405: F, t3188: F, t284: F, t10142: F, t876: F) -> (F, F, F, F, F, F, F) {
    let t10347 = t2207 * t190;
    let t10348 = t10346 * t10347;
    let t10349 = t442 * t875;
    let t10350 = t3439 * t10349;
    let t10351 = t10348 * t10350;
    let t10353 = t6939 * t190;
    let t10354 = t10346 * t10353;
    let t10355 = t786 * t19;
    let t10356 = t10355 * t147;
    let t10357 = t3296 * t10356;
    let t10358 = t10354 * t10357;
    let t10360 = t3188 * t2405;
    let t10361 = t284 * t10360;
    let t10363 = t10142 * t876;
    (t10349, t10350, t10351, t10357, t10358, t10361, t10363)
}
