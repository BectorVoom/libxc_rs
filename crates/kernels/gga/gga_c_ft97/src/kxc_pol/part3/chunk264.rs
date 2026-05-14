//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 264/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk264<F: Float>(t110: F, t447: F, t925: F, t452: F, t942: F, t464: F, t920: F, t463: F, t469: F, t24: F, t460: F, t462: F, t92: F, t457: F, t91: F, t477: F, t923: F, t945: F) -> (F, F, F, F, F, F, F, F) {
    let t951 = t447 * t110 * t925;
    let t955 = t452 * t110 * t942;
    let t958 = t464 * t920;
    let t959 = t463 * t958;
    let t962 = t469 * t942;
    let t963 = t24 * t962;
    let t965 = -t460 - t462 * t959 / 3.0 - t92 * t963;
    let t967 = t91 * t457 * t965;
    let t971 = t967 / 6.0 - t477 - t923 / 9.0 - t945 / 3.0;
    (t951, t955, t958, t959, t963, t965, t967, t971)
}
