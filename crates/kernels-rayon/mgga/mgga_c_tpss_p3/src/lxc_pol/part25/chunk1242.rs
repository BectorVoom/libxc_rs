//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1242/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1242(t1625: f64, t1659: f64, t1270: f64, t5371: f64, t5366: f64, t18439: f64, t5373: f64, t5377: f64, t5716: f64, t18446: f64, t5383: f64, t18454: f64, t5389: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21011 = t1625 * t1659;
    let t21017 = t1270 * t5371;
    let t21027 = t1270 * t5366;
    let t21036 = t18439 * t5373;
    let t21038 = t5716 * t5377;
    let t21040 = t18446 * t5383;
    let t21042 = t18454 * t5389;
    (t21011, t21017, t21027, t21036, t21038, t21040, t21042)
}
