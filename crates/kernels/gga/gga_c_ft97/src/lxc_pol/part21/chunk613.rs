//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 613/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk613<F: Float>(t1017: F, t604: F, t3447: F, t8392: F, t2097: F, t597: F, t3436: F, t3426: F, t3431: F, t1882: F, t3567: F, t12001: F, t3471: F, t3467: F, t12306: F, t12308: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12969 = t604 * t1017;
    let t12975 = 2.0 / 27.0 * t8392 * t3447;
    let t12982 = t2097 * t597;
    let t13040 = 4.0 / 27.0 * t8392 * t3436;
    let t13042 = 2.0 / 27.0 * t8392 * t3426;
    let t13049 = 2.0 / 27.0 * t8392 * t3431;
    let t13062 = 2.0 / 9.0 * t1882 * t3567;
    let t13075 = t12001 * t3471;
    let t13084 = 2.0 / 27.0 * t1882 * t3467;
    let t13100 = 2.0 / 9.0 * t12306;
    let t13101 = 4.0 / 9.0 * t12308;
    (t12969, t12975, t12982, t13040, t13042, t13049, t13062, t13075, t13084, t13100, t13101)
}
