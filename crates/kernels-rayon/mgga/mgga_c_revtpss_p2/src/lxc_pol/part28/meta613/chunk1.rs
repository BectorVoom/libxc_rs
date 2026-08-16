//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2142/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2142(t13435: f64, t7735: f64, t2322: f64, t27137: f64, t1453: f64, t1518: f64, t25800: f64, t28230: f64, t651: f64, t98567: f64, t98569: f64, t98571: f64, t98574: f64, t98578: f64, t98581: f64, t98584: f64, t98590: f64, t98594: f64, t98597: f64, t98599: f64, t98601: f64, t98603: f64, t98605: f64, t98607: f64) -> f64 {
    let t98609 = 4.0_f64 * t13435 * t7735;
    let t98611 = 4.0_f64 * t2322 * t27137;
    let t98612 = -2.0_f64 * t1518 * t25800 * t651 + 2.0_f64 * t1453 * t28230 + t98567 - t98569 - t98571 - t98574 + t98578 + t98581 - t98584 + t98590 + t98594 - t98597 - t98599 - t98601 - t98603 - t98605 - t98607 - t98609 - t98611;
    t98612
}
