//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 915/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk915(t17164: f64, t17167: f64, t17171: f64, t17175: f64, t17179: f64, t17187: f64, t17190: f64, t17193: f64, t17196: f64, t17200: f64, t17202: f64, t1648: f64, t5296: f64) -> (f64, f64) {
    let t17203 = t17164 + t17167 + t17171 + t17175 + t17179 + t17187 + t17190 + t17193 - t17196 - t17200 + t17202;
    let t17205 = 128.0_f64 / 81.0_f64 * t1648 * t5296;
    (t17203, t17205)
}
