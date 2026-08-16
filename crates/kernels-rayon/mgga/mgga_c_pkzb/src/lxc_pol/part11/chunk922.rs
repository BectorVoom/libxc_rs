//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 922/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk922(t10030: f64, t237: f64, t9863: f64, t9902: f64, t9984: f64, t154: f64, t907: f64, t9795: f64, t178: f64, t8358: f64, t2364: f64) -> (f64, f64, f64, f64) {
    let t10033 = t237 * (t9863 + t9902 + t9984 + t10030);
    let t10038 = t154 * t907 * t9795;
    let t10043 = t8358 * t178;
    let t10044 = t2364 * t10043;
    (t10033, t10038, t10043, t10044)
}
