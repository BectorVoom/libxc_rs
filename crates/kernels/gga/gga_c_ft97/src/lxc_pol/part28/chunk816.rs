//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 816/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk816<F: Float>(t1598: F, t1711: F, t22511: F, t22817: F, t3076: F, t1669: F, t22512: F, t2258: F, t69: F, t1642: F, t1800: F, t378: F, t8270: F, t463: F, t5704: F, t23823: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93014 = t1598 * t1711;
    let t93046 = t22817 * t22511;
    let t93047 = t3076 * t93046;
    let t93117 = t1669 * t93046;
    let t93157 = t1669 * t22512;
    let t93169 = t69 * t2258;
    let t93351 = t1642 * t1800;
    let t93355 = t378 * t8270;
    let t93636 = t463 * t5704;
    let t94400 = t23823 * t22511;
    (t93014, t93047, t93117, t93157, t93169, t93351, t93355, t93636, t94400)
}
