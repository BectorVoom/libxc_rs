//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1183/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1183(t20974: f64, t20975: f64, t20977: f64, t20978: f64, t20981: f64, t20982: f64, t20984: f64, t20989: f64, t369: f64, t6084: f64, t2100: f64, t931: f64) -> (f64, f64, f64) {
    let t20992 = t20974 + t20975 + t20977 + t20978 + t20981 + t20982 + t20984 + t20989;
    let t20995 = t6084 * t369;
    let t20998 = t2100 * t931;
    (t20992, t20995, t20998)
}
