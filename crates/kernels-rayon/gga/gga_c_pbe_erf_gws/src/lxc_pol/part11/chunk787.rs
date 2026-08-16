//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 787/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk787(t12766: f64, t1809: f64, t1620: f64, t2615: f64, t3415: f64, t12744: f64, t12746: f64, t12750: f64, t12754: f64, t12756: f64, t12758: f64, t12759: f64, t12760: f64, t12761: f64, t12763: f64, t12764: f64, t12765: f64, t5359: f64, t5948: f64, t5952: f64) -> (f64, f64, f64, f64) {
    let t12767 = t1809 * t12766;
    let t12769 = 8.0_f64 / 15.0_f64 * t1620 * t12767;
    let t12771 = 8.0_f64 / 15.0_f64 * t2615 * t3415;
    let t12772 = t5359 - t12744 + t12746 - t12750 + t12754 + t12756 + t5948 + t5952 + t12758 + t12759 - t12760 - t12761 + t12763 + t12764 + t12765 + t12769 - t12771;
    (t12767, t12769, t12771, t12772)
}
