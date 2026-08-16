//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1035/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1035(t14261: f64, t14262: f64, t14264: f64, t14265: f64, t219: f64, t1373: f64, t73: f64, t2387: f64, t4706: f64, t750: f64, t3610: f64, t3657: f64) -> (f64, f64, f64, f64) {
    let t14268 = (t14261 + t14262 + t14264 + t14265) * t219;
    let t14274 = t1373 * t73;
    let t14281 = t2387 * t4706;
    let t14282 = t14281 * t750;
    let t14285 = t3657 * t3610;
    (t14268, t14274, t14282, t14285)
}
