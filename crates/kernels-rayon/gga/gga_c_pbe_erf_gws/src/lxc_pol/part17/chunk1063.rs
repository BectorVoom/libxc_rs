//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1063/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1063(t2344: f64, t904: f64, t8828: f64, t1150: f64, t6717: f64, t8886: f64, t3219: f64, t3235: f64, t6360: f64, t875: f64, t9375: f64, t2343: f64, t3247: f64, t6714: f64, t6718: f64, t9181: f64, t9183: f64, t9187: f64, t9190: f64, t9192: f64, t9196: f64) -> (f64, f64, f64, f64, f64) {
    let t9665 = t2344 * t904;
    let t9666 = t9665 * t8828;
    let t9669 = t6717 * t1150;
    let t9671 = t9665 * t8886;
    let t9675 = t3235 * t3219 * t6360;
    let t9681 = t3235 * t9375 * t875;
    let t9684 = t2343 * t9666 / 192.0_f64 + 119.0_f64 / 6912.0_f64 * t9669 + t9181 - t3247 * t9671 / 64.0_f64 + t9183 + t9187 + t3247 * t9675 / 512.0_f64 - t9190 - t9192 - 7.0_f64 / 2304.0_f64 * t6714 + 119.0_f64 / 3456.0_f64 * t6718 - t9196 - t2343 * t9681 / 768.0_f64;
    (t9666, t9671, t9675, t9681, t9684)
}
