//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 683/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk683<F: Float>(t1983: F, t372: F, t2095: F, t355: F, t429: F, t1017: F, t604: F, t336: F, t578: F, t2016: F, t2056: F, t1156: F, t570: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7386 = t1983 * t372;
    let t7387 = t2095 * t7386;
    let t7388 = t7387 / F::cast_from(96.0_f64);
    let t7389 = t429 * t355;
    let t7390 = t2095 * t7389;
    let t7391 = F::cast_from(0.1528125e-1_f64) * t7390;
    let t7392 = t604 * t1017;
    let t7393 = t336 * t7392;
    let t7394 = t578 * t7393;
    let t7396 = t2016 * t2056;
    let t7397 = F::cast_from(0.28015625e-1_f64) * t7396;
    let t7398 = t570 * t1156;
    (t7386, t7387, t7388, t7389, t7390, t7391, t7393, t7394, t7396, t7397, t7398)
}
