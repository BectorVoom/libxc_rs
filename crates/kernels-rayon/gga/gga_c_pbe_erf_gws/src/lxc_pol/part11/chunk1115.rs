//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1115/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1115(t47782: f64, t47784: f64, t47786: f64, t47790: f64, t47793: f64, t47794: f64, t47795: f64, t47800: f64, t47801: f64, t47802: f64, t47805: f64, t47806: f64) -> f64 {
    let t47807 = t47782 - t47784 - t47786 - t47790 + t47793 + t47794 - t47795 + t47800 + t47801 + t47802 - t47805 + t47806;
    t47807
}
