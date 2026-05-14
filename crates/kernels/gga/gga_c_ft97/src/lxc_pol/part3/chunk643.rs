//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 643/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk643<F: Float>(t13811: F, t3951: F, t761: F, t1160: F, t737: F, t1144: F, t8232: F, t1882: F, t3991: F, t3899: F, t8392: F, t2372: F, t255: F, t1131: F, t3999: F, t3995: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13812 = 4.0 / 27.0 * t13811;
    let t13830 = t3951 * t761;
    let t13839 = t737 * t1160;
    let t13872 = t8232 * t1144;
    let t13875 = 2.0 / 9.0 * t1882 * t3991;
    let t13884 = 2.0 / 27.0 * t8392 * t3899;
    let t13885 = t2372 * t255;
    let t13886 = t761 * t1131;
    let t13903 = 2.0 / 9.0 * t1882 * t3999;
    let t13905 = 2.0 / 9.0 * t1882 * t3995;
    (t13812, t13830, t13839, t13872, t13875, t13884, t13885, t13886, t13903, t13905)
}
