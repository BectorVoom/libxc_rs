//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 788/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk788(t10887: f64, t10889: f64, t1024: f64, t10419: f64, t11005: f64, t950: f64, t5548: f64, t587: f64, t10505: f64, t954: f64, t1815: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12774 = 4.0_f64 / 15.0_f64 * t10887;
    let t12775 = 8.0_f64 / 15.0_f64 * t10889;
    let t12777 = 4.0_f64 / 5.0_f64 * t10419 * t1024;
    let t12778 = t11005 * t950;
    let t12779 = t5548 * t12778;
    let t12781 = 8.0_f64 / 15.0_f64 * t587 * t12779;
    let t12782 = t10505 * t954;
    let t12783 = t1815 * t12782;
    let t12785 = 4.0_f64 / 15.0_f64 * t639 * t12783;
    (t12774, t12775, t12777, t12778, t12779, t12781, t12782, t12783, t12785)
}
