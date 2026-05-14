//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 346/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk346<F: Float>(t1984: F, t2: F, t1956: F, t376: F, t599: F, t89: F, t597: F, t604: F, t161: F, t1637: F, t1882: F, t576: F, t611: F, t159: F, t603: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2112 = t1984 * t2;
    let t2124 = 4.0 / 27.0 * t1956;
    let t2140 = t89 * t376 * t599;
    let t2142 = t597 * t604;
    let t2149 = 4.0 / 9.0 * t1956;
    let t2164 = 4.0 / 27.0 * t89 * t1637 * t161;
    let t2165 = t1882 * t576;
    let t2167 = t1882 * t611;
    let t2178 = 1.0 / t603 / t159;
    (t2112, t2124, t2140, t2142, t2149, t2164, t2165, t2167, t2178)
}
