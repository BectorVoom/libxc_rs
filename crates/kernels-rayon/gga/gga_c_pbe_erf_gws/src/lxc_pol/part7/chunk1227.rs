//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1227/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1227(t20916: f64, t20919: f64, t20921: f64, t20964: f64, t20969: f64, t21039: f64, t21064: f64, t21068: f64, t21115: f64, t21123: f64, t21127: f64, t21155: f64, t21158: f64, t21174: f64, t21176: f64, t21183: f64, t21187: f64, t21191: f64, t21222: f64, t21224: f64, t21231: f64, t21239: f64) -> (f64, f64) {
    let t21702 = t20916 + t20919 - t20921 + t20964 + t20969 - t21039 - t21064 + t21068 + t21115 - t21123 - t21127;
    let t21704 = t21155 - t21158 + t21174 + t21176 + t21183 + t21187 - t21191 - t21222 + t21224 + t21231 - t21239;
    (t21702, t21704)
}
