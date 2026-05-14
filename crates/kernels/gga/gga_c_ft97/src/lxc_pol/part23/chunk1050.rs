//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1050/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1050<F: Float>(t296: F, t31360: F, t1212: F, t7131: F, t840: F, t1501: F, t19333: F, t31551: F, t319: F, t1508: F, t4973: F, t835: F, t2857: F, t4965: F, t7124: F, t871: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31752 = t296 * t31360;
    let t31758 = t840 * t7131 * t1212;
    let t31761 = t19333 * t1501;
    let t31762 = t296 * t31761;
    let t31766 = t840 * t319 * t31551;
    let t31770 = t835 * t1508 * t4973;
    let t31774 = t2857 * t1508 * t4965;
    let t31777 = t7124 * t1212;
    let t31779 = t840 * t871 * t31777;
    (t31752, t31758, t31761, t31762, t31766, t31770, t31774, t31777, t31779)
}
