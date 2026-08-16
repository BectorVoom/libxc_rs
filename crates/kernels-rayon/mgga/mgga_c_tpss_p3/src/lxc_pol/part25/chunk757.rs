//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 757/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk757(t1013: f64, t5072: f64, t128: f64, t2835: f64, t4044: f64, t5066: f64, t5070: f64, t408: f64, t1519: f64, t4063: f64, t1518: f64, t1043: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5073 = t1013 * t5072;
    let t5074 = t128 * t5073;
    let t5076 = t2835 - 0.11872222222222222222e-1_f64 * t4044 - 0.11872222222222222222e-1_f64 * t5066 + 0.35616666666666666666e-1_f64 * t5070 + 0.17808333333333333333e-1_f64 * t5074;
    let t5078 = 0.621814e-1_f64 * t5076 * t408;
    let t5080 = 2.0_f64 * t4063 * t1519;
    let t5081 = t1518 * t1518;
    let t5082 = t5081 * t1043;
    (t5073, t5074, t5076, t5078, t5080, t5081, t5082)
}
