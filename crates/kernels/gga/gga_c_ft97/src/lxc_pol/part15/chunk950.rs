//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 950/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk950<F: Float>(t20875: F, t8392: F, t20765: F, t1882: F, t20737: F, t20904: F, t20685: F, t20888: F, t20725: F, t20706: F, t20880: F, t20927: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77198 = t8392 * t20875;
    let t77214 = t8392 * t20765;
    let t77305 = t1882 * t20737;
    let t77307 = t1882 * t20904;
    let t77325 = t8392 * t20685;
    let t77346 = t1882 * t20888;
    let t77383 = t1882 * t20725;
    let t77386 = t1882 * t20706;
    let t77411 = t1882 * t20880;
    let t77452 = t8392 * t20927;
    (t77198, t77214, t77305, t77307, t77325, t77346, t77383, t77386, t77411, t77452)
}
