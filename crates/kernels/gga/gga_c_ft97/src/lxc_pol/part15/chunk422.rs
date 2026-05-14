//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 422/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk422<F: Float>(t1268: F, t2938: F, t113: F, t332: F, t1528: F, t920: F, t72: F, t942: F, t1524: F, t1526: F, t1527: F, t342: F, t343: F, t948: F, t947: F) -> (F, F, F, F, F, F, F) {
    let t4357 = t2938 * t1268;
    let t4381 = t332 * t113;
    let t4406 = t1528 * t920;
    let t4410 = t72 * t942;
    let t4414 = t948 - t1524 - t1526 * t1527 * t4406 / 12.0 - t342 * t343 * t4410 / 4.0;
    let t4415 = t4414 * t947;
    let t4417 = t920 * t920;
    (t4357, t4381, t4406, t4410, t4414, t4415, t4417)
}
