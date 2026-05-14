//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1092/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1092<F: Float>(t5829: F, t6608: F, t92433: F, t586: F, t7954: F, t3051: F, t5889: F, t2: F, t26768: F, t23649: F, t27167: F, t11176: F, t1368: F, t27144: F, t27149: F, t95053: F) -> (F, F, F, F, F, F, F, F) {
    let t105224 = 0.17780800291358024692e0 * t5829 * t92433 * t6608;
    let t105340 = t7954 * t586;
    let t105392 = t5889 * t3051;
    let t105406 = t2 * t26768;
    let t105411 = t23649 * t27167;
    let t105412 = t105411 / 9.0;
    let t105414 = t1368 * t11176 * t27144;
    let t105416 = t95053 * t27149;
    (t105224, t105340, t105392, t105406, t105411, t105412, t105414, t105416)
}
