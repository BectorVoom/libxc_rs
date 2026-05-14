//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 580/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk580<F: Float>(t1320: F, t3988: F, t1310: F, t1293: F, t1300: F, t1294: F, t1305: F, t20: F, t3913: F, t389: F, t1301: F, t172: F, t301: F, t342: F, t142: F, t416: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3989 = t1320 * t3988;
    let t3990 = t1310 * t3989;
    let t3993 = t1293 * t1300;
    let t3996 = t1294 * t1305;
    let t4000 = t3913 * t20;
    let t4001 = t389 * t4000;
    let t4004 = t1301 * t1305;
    let t4007 = t342 * t172 * t301;
    let t4008 = 0.23744444444444444444e-1 * t4007;
    let t4009 = t142 * t416;
    (t3989, t3990, t3993, t3996, t4000, t4001, t4004, t4007, t4008, t4009)
}
