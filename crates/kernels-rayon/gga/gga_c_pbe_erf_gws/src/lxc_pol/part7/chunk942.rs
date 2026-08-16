//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 942/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk942(t1872: f64, t5379: f64, t1879: f64, t5335: f64, t1672: f64, t1734: f64, t616: f64, t185: f64, t5178: f64, t582: f64, t172: f64, t184: f64, t4980: f64) -> (f64, f64, f64, f64, f64) {
    let t17531 = 8.0_f64 / 5.0_f64 * t5379 * t1872;
    let t17533 = 16.0_f64 / 15.0_f64 * t1879 * t5335;
    let t17535 = t616 * t1672 * t1734;
    let t17536 = 16.0_f64 / 45.0_f64 * t17535;
    let t17538 = t185 * t582 * t5178;
    let t17539 = 32.0_f64 / 15.0_f64 * t17538;
    let t17541 = t172 * t4980 * t184;
    (t17531, t17533, t17536, t17539, t17541)
}
