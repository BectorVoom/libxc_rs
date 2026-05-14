//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 635/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk635<F: Float>(t3431: F, t8392: F, t1882: F, t3567: F, t12001: F, t3471: F, t3467: F, t12306: F, t12308: F, t12310: F, t12327: F, t12356: F, t12365: F, t157: F, t526: F, t3421: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13049 = 2.0 / 27.0 * t8392 * t3431;
    let t13062 = 2.0 / 9.0 * t1882 * t3567;
    let t13075 = t12001 * t3471;
    let t13084 = 2.0 / 27.0 * t1882 * t3467;
    let t13100 = 2.0 / 9.0 * t12306;
    let t13101 = 4.0 / 9.0 * t12308;
    let t13102 = 4.0 / 27.0 * t12310;
    let t13108 = 2.0 / 9.0 * t12327;
    let t13117 = 4.0 / 3.0 * t12356;
    let t13120 = 2.0 / 3.0 * t12365;
    let t13140 = t526 * t157;
    let t13152 = 2.0 / 27.0 * t8392 * t3421;
    (t13049, t13062, t13075, t13084, t13100, t13101, t13102, t13108, t13117, t13120, t13140, t13152)
}
