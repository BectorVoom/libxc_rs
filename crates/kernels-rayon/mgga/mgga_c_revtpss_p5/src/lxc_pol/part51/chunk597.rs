//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 597/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk597(t3767: f64, t5330: f64, t1248: f64, t3603: f64, t5332: f64, t3720: f64, t1774: f64, t1250: f64, t1794: f64, t73: f64, t1214: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5340 = t3767 * t5330;
    let t5341 = t3603 * t1248;
    let t5342 = t5332 * t5341;
    let t5343 = t3720 * t5342;
    let t5346 = t1774 * t1248;
    let t5347 = t5346 * t1250;
    let t5348 = t3720 * t5347;
    let t5351 = t1794 * t73;
    let t5352 = t471 * t1214;
    (t5340, t5343, t5346, t5348, t5351, t5352)
}
