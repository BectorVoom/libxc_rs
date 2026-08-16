//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 985/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk985(t11132: f64, t1621: f64, t1620: f64, t3390: f64, t5109: f64, t661: f64, t639: f64, t2615: f64, t2689: f64, t2556: f64, t2562: f64, t11108: f64, t11109: f64, t11114: f64, t11118: f64, t11120: f64, t11122: f64, t11124: f64, t11128: f64, t11130: f64, t5562: f64, t7968: f64, t7970: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11133 = t1621 * t11132;
    let t11135 = 8.0_f64 / 15.0_f64 * t1620 * t11133;
    let t11136 = t5109 * t3390;
    let t11137 = t11136 * t661;
    let t11138 = t1621 * t11137;
    let t11140 = 4.0_f64 / 5.0_f64 * t639 * t11138;
    let t11142 = 8.0_f64 / 45.0_f64 * t2615 * t2689;
    let t11144 = 16.0_f64 / 45.0_f64 * t2615 * t2556;
    let t11146 = 8.0_f64 / 27.0_f64 * t2615 * t2562;
    let t11147 = t5562 - t11108 + t11109 + t11114 + t11118 + t11120 - t11122 + t11124 + t11128 + t11130 + t11135 - t11140 + t7968 + t7970 - t11142 - t11144 + t11146;
    (t11135, t11140, t11142, t11144, t11146, t11147)
}
