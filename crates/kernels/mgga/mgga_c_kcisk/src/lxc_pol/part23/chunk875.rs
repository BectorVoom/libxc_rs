//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 875/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk875<F: Float>(t3830: F, t423: F, t1407: F, t3805: F, t1333: F, t3916: F, t3919: F, t13959: F, t3800: F, t3734: F, t3739: F, t1404: F, t3783: F, t3513: F, t1299: F, t3795: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14140 = 1.0 / t3830 / t423;
    let t14160 = t3805 * t1407;
    let t14162 = t1333 * t3916;
    let t14173 = t1333 * t3919;
    let t14179 = t13959 * t3800;
    let t14181 = t3739 * t3734;
    let t14187 = t1404 * t3783;
    let t14188 = t14187 * sigma0;
    let t14195 = t3739 * t3513;
    let t14199 = t3795 * t1299;
    (t14140, t14160, t14162, t14173, t14179, t14181, t14187, t14188, t14195, t14199)
}
