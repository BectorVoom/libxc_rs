//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 807/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk807(t2938: f64, t6366: f64, t2960: f64, t6320: f64, t6338: f64, t939: f64, t2970: f64, t6326: f64, t26: f64, t6330: f64, t945: f64, t6334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6368 = 2.0_f64 * t2938 * t6366;
    let t6375 = t2960 * t6320;
    let t6377 = t939 * t6338;
    let t6380 = t2970 * t6326;
    let t6381 = t26 * t6380;
    let t6383 = t945 * t6330;
    let t6384 = t26 * t6383;
    let t6386 = t945 * t6334;
    (t6368, t6375, t6377, t6380, t6381, t6383, t6384, t6386)
}
