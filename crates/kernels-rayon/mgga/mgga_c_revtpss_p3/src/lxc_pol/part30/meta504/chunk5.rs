//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1883/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1883(t118: f64, t1310: f64, t1453: f64, t2163: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t25085: f64, t25092: f64, t25095: f64, t25180: f64, t25182: f64, t25184: f64, t25186: f64, t25189: f64, t26800: f64, t26804: f64, t27056: f64, t27066: f64, t508: f64, t569: f64, t649: f64, t7584: f64, t7586: f64, t7591: f64, t7683: f64, t7687: f64) -> f64 {
    let t27075 = -t118 * t27056 - 2.0_f64 * t1310 * t7584 + 2.0_f64 * t1453 * t7687 - t2163 * t2320 - 2.0_f64 * t2163 * t2328 - 4.0_f64 * t2322 * t7591 - 4.0_f64 * t2331 * t7586 - t26800 * t508 - 2.0_f64 * t26804 * t508 + t27066 * t569 - 2.0_f64 * t649 * t7683 - t25085 + t25092 - t25095 + t25180 - t25182 - t25184 - t25186 + t25189;
    t27075
}
