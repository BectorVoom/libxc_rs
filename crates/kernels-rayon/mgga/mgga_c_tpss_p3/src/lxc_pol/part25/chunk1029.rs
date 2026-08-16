//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1029/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1029(t14245: f64, t2389: f64, t774: f64, t10617: f64, t10620: f64, t10630: f64, t10635: f64, t10642: f64, t14220: f64, t14223: f64, t14229: f64, t14234: f64, t14238: f64, t14242: f64, t2173: f64, t3626: f64, t797: f64, t8131: f64) -> (f64, f64) {
    let t14247 = t2389 * t774 * t14245;
    let t14250 = -7.0_f64 / 576.0_f64 * t14220 + t3626 * t14223 / 1536.0_f64 - 119.0_f64 / 1728.0_f64 * t10617 + t10620 - 119.0_f64 / 3456.0_f64 * t8131 + t2173 * t14229 / 384.0_f64 + t2173 * t14234 / 384.0_f64 + t10630 - 35.0_f64 / 108.0_f64 * t10635 - t10642 + 7.0_f64 / 4608.0_f64 * t14238 - 5.0_f64 / 128.0_f64 * t797 * t14242 + 5.0_f64 / 384.0_f64 * t797 * t14247;
    (t14247, t14250)
}
