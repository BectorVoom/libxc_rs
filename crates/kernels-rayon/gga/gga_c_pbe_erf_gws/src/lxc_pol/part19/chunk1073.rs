//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1073/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1073(t12065: f64, t8848: f64, t1139: f64, t2169: f64, t856: f64, t3108: f64, t1076: f64, t1112: f64, t2118: f64, t3074: f64, t1185: f64, t346: f64, t825: f64) -> (f64, f64, f64, f64, f64) {
    let t12067 = t8848 * t12065 / 96.0_f64;
    let t12068 = t2169 * t1139;
    let t12069 = t856 * t12068;
    let t12071 = t3108 * t12069 / 24.0_f64;
    let t12072 = t1112 * t1076;
    let t12073 = t2118 * t12072;
    let t12074 = t3074 * t12073;
    let t12076 = t346 * t825 * t1185;
    (t12067, t12071, t12072, t12074, t12076)
}
