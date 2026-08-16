//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 972/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk972(t17877: f64, t18008: f64, t163: f64, t169: f64, t234: f64, t922: f64, t1354: f64, t784: f64, t1378: f64, t1971: f64, t4585: f64, t5701: f64) -> (f64, f64, f64, f64, f64) {
    let t18009 = t17877 + t18008;
    let t18021 = 0.40978489723982440011e0_f64 * t169 * t922 * t234 * t163;
    let t18022 = t784 * t1354;
    let t18024 = t18022 * t1378 * t1971;
    let t18027 = t5701 * t4585 * t1971;
    (t18009, t18021, t18022, t18024, t18027)
}
