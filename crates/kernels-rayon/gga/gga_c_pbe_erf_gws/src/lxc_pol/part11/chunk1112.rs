//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1112/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1112(t12778: f64, t23336: f64, t5218: f64, t12813: f64, t7495: f64, t31879: f64, t1019: f64, t12486: f64, t1046: f64, t12528: f64, t18215: f64, t47760: f64, t47761: f64, t47762: f64, t47765: f64, t47769: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47772 = 64.0_f64 / 15.0_f64 * t5218 * t23336 * t12778;
    let t47775 = 64.0_f64 / 15.0_f64 * t5218 * t7495 * t12813;
    let t47776 = 16.0_f64 / 45.0_f64 * t31879;
    let t47778 = 8.0_f64 / 15.0_f64 * t12486 * t1019;
    let t47780 = 16.0_f64 / 5.0_f64 * t12528 * t1046;
    let t47781 = t47760 - t47761 + t47762 + t47765 - t47769 + t47772 + t47775 + t18215 + t47776 - t47778 - t47780;
    (t47772, t47775, t47776, t47778, t47780, t47781)
}
