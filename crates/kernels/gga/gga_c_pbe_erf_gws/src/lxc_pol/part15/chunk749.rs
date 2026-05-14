//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 749/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk749<F: Float>(t2395: F, t810: F, t2370: F, t830: F, t2417: F, t3067: F, t829: F, t2209: F, t337: F, t2182: F, t831: F, t2118: F, t2365: F, t822: F, t2362: F, t274: F, t4394: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6133 = t2395 * t810;
    let t6135 = t2370 * t830 * t6133;
    let t6143 = t3067 * t2417;
    let t6145 = t829 * t830 * t6143;
    let t6148 = t2209 * t337;
    let t6149 = t831 * t2182;
    let t6151 = t6148 * t830 * t6149;
    let t6154 = t2118 * t2365;
    let t6155 = t822 * t6154;
    let t6156 = t6155 * t2362;
    let t6158 = t4394 * t274;
    (t6133, t6135, t6143, t6145, t6148, t6149, t6151, t6154, t6156, t6158)
}
