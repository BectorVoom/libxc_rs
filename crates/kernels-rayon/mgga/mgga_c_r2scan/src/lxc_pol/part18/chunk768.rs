//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 768/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk768(t2271: f64, t2813: f64, t2452: f64, t410: f64, t2484: f64, t406: f64, t1416: f64, t899: f64, t1419: f64, t2483: f64, t457: f64, t41: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7050 = 0.4726e1_f64 * t2271 * t2813;
    let t7051 = t410 * t2452;
    let t7094 = t406 * t2484;
    let t7095 = 8.0_f64 * t7094;
    let t7096 = t410 * t2484;
    let t7097 = 8.0_f64 * t7096;
    let t7109 = t1416 * t899;
    let t7111 = t1419 * t899;
    let t7124 = t2483 * t457;
    let t7125 = t41 * t7124;
    (t7050, t7051, t7095, t7097, t7109, t7111, t7125)
}
