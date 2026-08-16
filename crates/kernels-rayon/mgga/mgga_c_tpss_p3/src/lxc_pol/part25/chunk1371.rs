//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1371/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1371(t1692: f64, t1812: f64, t18728: f64, t18807: f64, t20012: f64, t20041: f64, t20058: f64, t20417: f64, t20526: f64, t21495: f64, t21516: f64, t21659: f64, t2439: f64, t33: f64, t3552: f64, t5671: f64, t5849: f64, t6214: f64, t6354: f64, t66281: f64, t66317: f64, t70847: f64, t70890: f64, t70915: f64, t70923: f64, t70942: f64, t70957: f64, t72173: f64, t72187: f64, t72279: f64) -> f64 {
    let t72495 = 3.0_f64 / 2.0_f64 * t2439 * t21659 * t5671 - 3.0_f64 * t18728 * t70847 + 3.0_f64 * t3552 * t1812 * t70923 - t1692 * t18807 * t21516 / 2.0_f64 - 3.0_f64 * t18728 * t70890 - 3.0_f64 / 2.0_f64 * t18728 * t70915 + 2.0_f64 * t20526 * t70942 + 3.0_f64 * t2439 * t5849 * t21495 - 3.0_f64 * t20417 * t70957 + t72187 + 6.0_f64 * t72279 * t20012 + t1692 * t72173 * t33 / 2.0_f64 - 3.0_f64 * t66317 * t20041 - t1692 * t66281 * t6214 + 3.0_f64 * t2439 * t6354 * t20058;
    t72495
}
