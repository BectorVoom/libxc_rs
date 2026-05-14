//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 989/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk989<F: Float>(t222: F, t3722: F, t2379: F, t13417: F, t7853: F, t2378: F, t37481: F, t13412: F, t13444: F, t3817: F, t709: F, t1127: F, t2455: F, t17839: F, t3771: F, t6041: F) -> (F, F, F, F, F, F, F, F) {
    let t66382 = t3722 * t222;
    let t66383 = t2379 * t66382;
    let t66416 = t7853 * t13417;
    let t66422 = t37481 * t2378;
    let t66423 = t66422 * t13412;
    let t66556 = t7853 * t13444;
    let t66612 = t709 * t3817;
    let t66619 = t1127 * t2455;
    let t66680 = t3771 * t6041 * t17839;
    (t66383, t66416, t66422, t66423, t66556, t66612, t66619, t66680)
}
