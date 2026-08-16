//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 844/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk844(t185: f64, t7121: f64, t1627: f64, t2667: f64, t2674: f64, t2680: f64, t2789: f64, t586: f64, t1824: f64, t1829: f64, t2615: f64, t7083: f64, t7084: f64, t7086: f64, t7091: f64, t7096: f64, t7100: f64, t7101: f64, t7105: f64, t7109: f64, t7113: f64, t7120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7122 = t185 * t7121;
    let t7123 = 4.0_f64 / 135.0_f64 * t7122;
    let t7125 = 8.0_f64 / 45.0_f64 * t1627 * t2667;
    let t7127 = 16.0_f64 / 45.0_f64 * t1627 * t2674;
    let t7129 = 8.0_f64 / 27.0_f64 * t1627 * t2680;
    let t7130 = t2789 * t586;
    let t7132 = 16.0_f64 / 45.0_f64 * t7130 * t1824;
    let t7134 = 8.0_f64 / 45.0_f64 * t2615 * t1829;
    let t7135 = -t7083 - t7084 - t7086 - t7091 - t7096 + t7100 + t7101 + t7105 - t7109 - t7113 + t7120 + t7123 - t7125 - t7127 + t7129 + t7132 - t7134;
    (t7123, t7125, t7127, t7129, t7130, t7132, t7134, t7135)
}
