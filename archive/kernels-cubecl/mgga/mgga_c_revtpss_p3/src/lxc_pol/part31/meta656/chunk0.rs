//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2205/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2205<F: Float>(t1444: F, t6862: F, t22107: F, t26028: F, t22111: F, t22271: F, t27940: F, t22163: F, t6871: F, t94429: F, t22159: F, t98115: F) -> (F, F, F, F, F, F, F) {
    let t108502 = t6862 * t1444;
    let t108508 = t26028 * t22107;
    let t108510 = t26028 * t22111;
    let t108512 = t27940 * t22271;
    let t108514 = t27940 * t22163;
    let t108516 = t94429 * t6871;
    let t108518 = t98115 * t22159;
    (t108502, t108508, t108510, t108512, t108514, t108516, t108518)
}
