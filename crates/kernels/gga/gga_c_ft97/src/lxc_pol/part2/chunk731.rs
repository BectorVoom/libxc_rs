//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 731/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk731<F: Float>(t3436: F, t8392: F, t3426: F, t1986: F, t920: F, t2222: F, t9133: F, t3431: F, t1647: F, t3419: F, t2210: F, t11437: F, t3440: F, t12603: F, t144: F, t1882: F, t3567: F) -> (F, F, F, F, F, F, F, F) {
    let t13040 = 4.0 / 27.0 * t8392 * t3436;
    let t13042 = 2.0 / 27.0 * t8392 * t3426;
    let t13043 = t920 * t1986;
    let t13044 = t2222 * t13043;
    let t13045 = t9133 * t13044;
    let t13049 = 2.0 / 27.0 * t8392 * t3431;
    let t13050 = t3419 * t1647;
    let t13051 = t2210 * t13050;
    let t13054 = t3440 * t11437;
    let t13055 = t2210 * t13054;
    let t13058 = t144 * t12603;
    let t13062 = 2.0 / 9.0 * t1882 * t3567;
    (t13040, t13042, t13045, t13049, t13051, t13055, t13058, t13062)
}
