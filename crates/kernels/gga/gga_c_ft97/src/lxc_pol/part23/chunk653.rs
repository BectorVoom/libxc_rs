//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 653/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk653<F: Float>(t4152: F, t8392: F, t1882: F, t4173: F, t4188: F, t4178: F, t4183: F, t4267: F, t339: F, t39: F, t11: F, t340: F, t14: F, t1577: F, t7743: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15467 = 2.0 / 27.0 * t8392 * t4152;
    let t15471 = 2.0 / 27.0 * t1882 * t4173;
    let t15491 = 2.0 / 27.0 * t1882 * t4188;
    let t15500 = 2.0 / 9.0 * t1882 * t4178;
    let t15502 = 4.0 / 9.0 * t1882 * t4183;
    let t15532 = 4.0 / 27.0 * t8392 * t4267;
    let t15564 = t339 * t39;
    let t15565 = t340 * t11;
    let t15567 = t15564 * t15565 * t14;
    let t15625 = -2.0 * t1577 - 6.0 * t7743;
    let t16579 = -t15625;
    (t15467, t15471, t15491, t15500, t15502, t15532, t15564, t15565, t15567, t15625, t16579)
}
