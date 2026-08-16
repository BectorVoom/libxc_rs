//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 986/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk986(t7742: f64, t8634: f64, t4147: f64, t7933: f64, t2034: f64, t2014: f64, t7937: f64, t8568: f64, t32098: f64, t7900: f64, t1519: f64, t32162: f64, t33575: f64, t33578: f64, t33580: f64, t33583: f64, t33584: f64, t33587: f64, t33589: f64, t33592: f64, t33595: f64, t33599: f64, t33600: f64, t33603: f64, t33605: f64, t33647: f64, t569: f64, t651: f64) -> (f64, f64, f64, f64) {
    let t33650 = 4.0_f64 * t8634 * t7742;
    let t33651 = t4147 * t7933;
    let t33652 = t2034 * t33651;
    let t33654 = 2.0_f64 * t2014 * t33652;
    let t33655 = t8568 * t7937;
    let t33657 = t32098 * t7900;
    let t33659 = 3.0_f64 * t2014 * t33657;
    let t33660 = -2.0_f64 * t1519 * t32162 - 2.0_f64 * t33584 * t651 + t33647 * t569 - 4.0_f64 * t33575 - t33578 - t33580 - t33583 - 4.0_f64 * t33587 - 4.0_f64 * t33589 - 4.0_f64 * t33592 - t33595 - t33599 - 4.0_f64 * t33600 - 4.0_f64 * t33603 - 4.0_f64 * t33605 - t33650 - t33654 - 2.0_f64 * t33655 + t33659;
    (t33651, t33652, t33657, t33660)
}
