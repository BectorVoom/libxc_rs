//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 576/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk576<F: Float>(t1620: F, t8010: F, t1615: F, t1630: F, t1608: F, t1619: F, t1681: F, t1655: F, t383: F, t35: F, t1594: F, t1632: F, t428: F) -> (F, F, F, F, F, F, F, F) {
    let t8011 = t8010 * t1620;
    let t8014 = t1615 * t1630;
    let t8015 = t1608 * t8014;
    let t8018 = t1619 * t1681;
    let t8030 = t1655 * t383;
    let t8031 = t8030 * t35;
    let t8032 = t1594 * t8031;
    let t8035 = t1632 * t428;
    (t8011, t8014, t8015, t8018, t8030, t8031, t8032, t8035)
}
