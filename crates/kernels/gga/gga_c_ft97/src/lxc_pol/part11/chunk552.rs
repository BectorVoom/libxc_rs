//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 552/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk552<F: Float>(t1608: F, t8014: F, t1619: F, t1681: F, t1655: F, t383: F, t35: F, t1594: F, t1632: F, t428: F, t1631: F, t1711: F, t371: F, t1712: F, t384: F, t374: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8015 = t1608 * t8014;
    let t8018 = t1619 * t1681;
    let t8030 = t1655 * t383;
    let t8031 = t8030 * t35;
    let t8032 = t1594 * t8031;
    let t8035 = t1632 * t428;
    let t8036 = t1594 * t8035;
    let t8039 = t1631 * t8031;
    let t8042 = t371 * t1711;
    let t8043 = t384 * t1712;
    let t8044 = t374 * t8043;
    (t8015, t8018, t8030, t8031, t8032, t8035, t8036, t8039, t8042, t8044)
}
