//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1336/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1336<F: Float>(t24980: F, t25165: F, t2862: F, t31600: F, t1212: F, t4226: F, t6318: F, t1234: F, t4129: F, t113350: F, t5337: F, t840: F, t856: F, t113101: F, t18997: F, t1901: F) -> (F, F, F, F, F) {
    let t126697 = t24980 * t2862 * t25165 * t31600;
    let t126701 = t24980 * t2862 * t6318 * t4226 * t1212;
    let t126705 = t24980 * t2862 * t6318 * t1234 * t4129;
    let t126709 = t113350 * t840 * t6318 * t5337 * t856;
    let t126712 = t1901 * t113101 * t18997;
    (t126697, t126701, t126705, t126709, t126712)
}
