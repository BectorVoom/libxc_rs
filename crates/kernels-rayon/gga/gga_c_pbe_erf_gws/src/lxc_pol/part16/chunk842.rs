//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 842/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk842(t4999: f64, t5020: f64, t1627: f64, t2685: f64, t1764: f64, t995: f64, t1403: f64, t1821: f64, t1820: f64, t5002: f64, t954: f64, t1413: f64) -> (f64, f64, f64, f64, f64) {
    let t7083 = 16.0_f64 / 135.0_f64 * t4999;
    let t7084 = 16.0_f64 / 45.0_f64 * t5020;
    let t7086 = 8.0_f64 / 45.0_f64 * t1627 * t2685;
    let t7087 = t995 * t1764;
    let t7088 = t7087 * t1403;
    let t7089 = t1821 * t7088;
    let t7091 = 16.0_f64 / 45.0_f64 * t1820 * t7089;
    let t7092 = t5002 * t954;
    let t7093 = t7092 * t1413;
    (t7083, t7084, t7086, t7091, t7093)
}
