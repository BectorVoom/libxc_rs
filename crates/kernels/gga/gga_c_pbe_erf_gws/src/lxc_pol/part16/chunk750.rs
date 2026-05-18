//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 750/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk750<F: Float>(t1917: F, t528: F, t1775: F, t583: F, t220: F, t2735: F, t211: F, t1750: F, t636: F, t1729: F, t586: F, t1791: F, t642: F) -> (F, F, F, F, F, F, F) {
    let t4876 = F::new(0.9973633333333333333e-1) * t528 * t1917;
    let t4906 = t1775 * t583;
    let t4908 = t2735 * t220;
    let t4910 = F::new(16.0) / F::new(405.0) * t211 * t4908;
    let t4911 = t1750 * t636;
    let t4913 = t1729 * t586;
    let t4927 = t642 * t1791;
    (t4876, t4906, t4908, t4910, t4911, t4913, t4927)
}
