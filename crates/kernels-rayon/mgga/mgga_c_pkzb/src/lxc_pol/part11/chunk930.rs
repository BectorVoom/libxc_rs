//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 930/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk930(t10075: f64, t10121: f64, t406: f64, t154: f64, t3757: f64, t6431: f64, t385: f64, t2347: f64, t3730: f64, t1220: f64, t3171: f64, t2099: f64, t3876: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10122 = t10075 * t10121;
    let t10123 = t406 * t10122;
    let t10131 = t154 * t6431 * t3757;
    let t10132 = t385 * t10131;
    let t10135 = t154 * t2347 * t3730;
    let t10136 = t385 * t10135;
    let t10138 = t1220 * t3171;
    let t10140 = t2099 * t3876;
    (t10122, t10123, t10131, t10132, t10135, t10136, t10138, t10140)
}
