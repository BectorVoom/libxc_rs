//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 907/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk907(t17105: f64, t108: f64, t1729: f64, t267: f64, t5214: f64, t1733: f64, t1816: f64, t5211: f64, t5212: f64, t4897: f64, t5213: f64, t5145: f64) -> (f64, f64, f64, f64, f64) {
    let t17106 = 128.0_f64 / 405.0_f64 * t17105;
    let t17108 = t1729 * t108 * t267;
    let t17110 = 64.0_f64 / 15.0_f64 * t17108 * t5214;
    let t17114 = 32.0_f64 / 15.0_f64 * t5211 * t5212 * t1733 * t1816;
    let t17117 = 32.0_f64 / 15.0_f64 * t5211 * t5213 * t4897;
    let t17120 = 32.0_f64 / 15.0_f64 * t5211 * t5213 * t5145;
    (t17106, t17110, t17114, t17117, t17120)
}
