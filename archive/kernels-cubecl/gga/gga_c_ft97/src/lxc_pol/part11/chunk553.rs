//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 553/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk553<F: Float>(t1866: F, t7807: F, t446: F, t1651: F, t432: F, t1564: F, t1755: F, t379: F, t1546: F, t1572: F, t89: F, t1566: F, t1882: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7808 = t1866 * t7807;
    let t7809 = t446 * t7808;
    let t7811 = t1651 * t432;
    let t7812 = t1564 * t7811;
    let t7813 = t446 * t7812;
    let t7815 = t379 * t1755;
    let t7816 = t1564 * t7815;
    let t7817 = t446 * t7816;
    let t7820 = t89 * t1546 * t1572;
    let t7822 = t1882 * t1566;
    (t7808, t7809, t7811, t7812, t7813, t7815, t7816, t7817, t7820, t7822)
}
