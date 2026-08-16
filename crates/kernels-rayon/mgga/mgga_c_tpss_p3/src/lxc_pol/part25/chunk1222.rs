//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1222/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1222(t1322: f64, t1600: f64, t1760: f64, t1796: f64, t1830: f64, t18547: f64, t19620: f64, t20134: f64, t20137: f64, t20219: f64, t20221: f64, t20224: f64, t20227: f64, t20322: f64, t3491: f64, t4341: f64, t544: f64, t5706: f64, t5799: f64, t5895: f64, t5910: f64, t5939: f64, t6243: f64, t6413: f64) -> f64 {
    let t20329 = -t1322 * t5895 - t1600 * t5799 + 3.0_f64 * t1760 * t20137 + t1760 * t20219 - t1760 * t20224 + 3.0_f64 * t1760 * t20227 - t1796 * t4341 - t1830 * t3491 - 3.0_f64 * t18547 * t20221 + 6.0_f64 * t19620 * t20134 + t20322 * t544 + 3.0_f64 * t5706 * t6413 + 3.0_f64 * t5910 * t6243 - t5939 * t6243;
    t20329
}
