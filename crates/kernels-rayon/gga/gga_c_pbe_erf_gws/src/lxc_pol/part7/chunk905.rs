//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 905/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk905(t17082: f64, t17058: f64, t17063: f64, t17067: f64, t17069: f64, t17071: f64, t17073: f64, t17075: f64, t17077: f64, t17079: f64, t17081: f64, t1661: f64, t16669: f64, t5294: f64, t587: f64) -> (f64, f64, f64) {
    let t17083 = 16.0_f64 / 45.0_f64 * t17082;
    let t17084 = -t17058 - t17063 + t17067 - t17069 - t17071 + t17073 - t17075 + t17077 + t17079 - t17081 - t17083;
    let t17090 = 16.0_f64 / 3.0_f64 * t587 * t1661 * t5294 * t16669;
    (t17083, t17084, t17090)
}
