//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1557/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1557(t21213: f64, t5357: f64, t1256: f64, t24681: f64, t24671: f64, t21233: f64, t5391: f64, t1261: f64, t24240: f64, t247: f64, t3634: f64, t21192: f64, t5381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83316 = t21213 * t5357;
    let t83369 = t24681 * t1256;
    let t83371 = t24671 * t1256;
    let t83382 = t5391 * t21233;
    let t83392 = t1261 * t247 * t3634 * t24240;
    let t83394 = t5381 * t21192;
    (t83316, t83369, t83371, t83382, t83392, t83394)
}
