//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 595/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk595<F: Float>(t81: F, t8633: F, t2258: F, t342: F, t4410: F, t630: F, t4436: F, t7241: F, t4418: F, t7780: F, t89: F, t1546: F, t4426: F, t4432: F, t1597: F, t4441: F) -> (F, F, F, F, F, F, F, F) {
    let t15568 = t8633 * t81;
    let t15575 = t2258 * t81;
    let t15584 = t342 * t630 * t4410;
    let t15601 = t7241 * t4436;
    let t15606 = t89 * t7780 * t4418;
    let t15609 = t89 * t1546 * t4426;
    let t15612 = t89 * t1546 * t4432;
    let t15630 = t4441 * t1597;
    (t15568, t15575, t15584, t15601, t15606, t15609, t15612, t15630)
}
