//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 330/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk330<F: Float>(t72: F, t942: F, t1524: F, t1526: F, t1527: F, t342: F, t343: F, t4406: F, t948: F, t1943: F, t920: F, t1017: F) -> (F, F, F, F) {
    let t4410 = t72 * t942;
    let t4414 = t948 - t1524 - t1526 * t1527 * t4406 / F::cast_from(12.0_f64) - t342 * t343 * t4410 / F::cast_from(4.0_f64);
    let t4641 = t1943 * t920;
    let t4645 = t72 * t1017;
    (t4410, t4414, t4641, t4645)
}
