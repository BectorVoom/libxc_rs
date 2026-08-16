//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1277/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1277(t18005: f64, t5567: f64, t1706: f64, t5570: f64, t8347: f64, t1006: f64, t2436: f64, t18546: f64, t5705: f64, t112: f64, t789: f64, t234: f64, t630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61222 = t5567 * t18005;
    let t61226 = t1706 * t5570 * t8347;
    let t61703 = t2436 * t1006;
    let t61801 = t5705 * t18546;
    let t61868 = t789 * t112;
    let t61870 = t234 * t630;
    (t61222, t61226, t61703, t61801, t61868, t61870)
}
