//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 990/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk990(t312: f64, t4129: f64, t684: f64, t2874: f64, t14075: f64, t4140: f64, t4139: f64, t4299: f64, t870: f64, t2881: f64, t13863: f64, t10447: f64, t4266: f64) -> (f64, f64, f64, f64, f64) {
    let t15344 = t312 * t4129;
    let t15345 = t15344 * t684;
    let t15346 = t2874 * t15345;
    let t15349 = t4140 * t14075;
    let t15350 = t4139 * t15349;
    let t15353 = t870 * t4299;
    let t15354 = t15353 * t684;
    let t15355 = t2881 * t15354;
    let t15358 = t4140 * t13863;
    let t15359 = t2881 * t15358;
    let t15362 = t10447 * t4266;
    (t15346, t15350, t15355, t15359, t15362)
}
