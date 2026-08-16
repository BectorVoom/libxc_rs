//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 785/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk785(t1046: f64, t3479: f64, t10969: f64, t997: f64, t3351: f64, t7651: f64, t1809: f64, t1620: f64, t1044: f64, t3469: f64, t1815: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12744 = 2.0_f64 / 5.0_f64 * t3479 * t1046;
    let t12746 = 4.0_f64 / 5.0_f64 * t10969 * t997;
    let t12747 = t7651 * t3351;
    let t12748 = t1809 * t12747;
    let t12750 = 16.0_f64 / 15.0_f64 * t1620 * t12748;
    let t12751 = t3469 * t1044;
    let t12752 = t1815 * t12751;
    let t12754 = 8.0_f64 / 15.0_f64 * t639 * t12752;
    (t12744, t12746, t12747, t12748, t12750, t12751, t12752, t12754)
}
