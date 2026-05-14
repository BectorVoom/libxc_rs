//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 954/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk954<F: Float>(t1307: F, t4589: F, t452: F, t488: F, t6454: F, t979: F, t3238: F, t6478: F, t110: F, t1871: F, t29701: F, t6469: F, t986: F, t29706: F, t29608: F, t83: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29956 = t1307 * t4589;
    let t29958 = t452 * t488 * t29956;
    let t29961 = t6454 * t979;
    let t29963 = t452 * t488 * t29961;
    let t29967 = t452 * t3238 * t6478;
    let t29971 = t1871 * t110 * t29701;
    let t29975 = t1871 * t986 * t6469;
    let t29979 = t1871 * t110 * t29706;
    let t29982 = t83 * t29608;
    (t29956, t29958, t29961, t29963, t29967, t29971, t29975, t29979, t29982)
}
