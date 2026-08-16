//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 236/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk236<F: Float>(t110: F, t447: F, t925: F, t452: F, t942: F, t464: F, t920: F, t463: F, t469: F, t24: F, t460: F, t462: F, t92: F) -> (F, F, F, F, F, F) {
    let t951 = t447 * t110 * t925;
    let t955 = t452 * t110 * t942;
    let t958 = t464 * t920;
    let t959 = t463 * t958;
    let t962 = t469 * t942;
    let t963 = t24 * t962;
    let t965 = -t460 - t462 * t959 / F::cast_from(3.0_f64) - t92 * t963;
    (t951, t955, t958, t959, t963, t965)
}
