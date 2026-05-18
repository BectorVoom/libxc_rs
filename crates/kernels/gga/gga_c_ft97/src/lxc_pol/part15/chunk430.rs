//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 430/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk430<F: Float>(t1524: F, t1526: F, t1527: F, t342: F, t343: F, t4406: F, t4410: F, t948: F, t947: F, t920: F) -> (F, F, F) {
    let t4414 = t948 - t1524 - t1526 * t1527 * t4406 / F::new(12.0) - t342 * t343 * t4410 / F::new(4.0);
    let t4415 = t4414 * t947;
    let t4417 = t920 * t920;
    (t4414, t4415, t4417)
}
