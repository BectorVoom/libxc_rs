//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 803/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk803(t10134: f64, t12962: f64, t12970: f64, t12973: f64, t12987: f64, t138: f64, t1577: f64, t2902: f64, t3675: f64, t3683: f64, t514: f64, t5854: f64, t8209: f64, t985: f64) -> f64 {
    let t12989 = -3.0_f64 * t10134 * t985 + t12962 * t138 - 6.0_f64 * t12970 * t5854 + 6.0_f64 * t12973 * t1577 - t12987 * t514 - 3.0_f64 * t2902 * t3683 + 6.0_f64 * t3675 * t8209;
    t12989
}
