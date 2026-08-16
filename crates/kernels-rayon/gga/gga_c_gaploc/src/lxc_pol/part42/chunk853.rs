//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 853/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk853(t43407: f64, t2617: f64, t3621: f64, t7803: f64, t43412: f64, t43416: f64, t15499: f64, t3601: f64, t2679: f64, t28640: f64, t10827: f64, t3005: f64, t9800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45195 = 0.92023022289409799224e1_f64 * t43407;
    let t45197 = t7803 * t3621 * t2617;
    let t45199 = 0.15337170381568299871e1_f64 * t43412;
    let t45200 = 0.15337170381568299871e1_f64 * t43416;
    let t45209 = t15499 * t3601;
    let t45211 = t28640 * t45209 * t2679;
    let t45212 = 0.23005755572352449806e1_f64 * t45211;
    let t45214 = t9800 * t3005 * t10827;
    (t45195, t45197, t45199, t45200, t45212, t45214)
}
