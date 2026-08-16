//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2262/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2262(t17633: f64, t3629: f64, t3626: f64, t2258: f64, t3628: f64, t5351: f64, t3367: f64, t471: f64, t2251: f64, t372: f64, t5296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17634 = t17633 * t3629;
    let t17635 = t3626 * t17634;
    let t17638 = t3628 * t2258;
    let t17639 = t5351 * t17638;
    let t17640 = t3626 * t17639;
    let t17643 = t471 * t3367;
    let t17644 = t17643 * t2251;
    let t17645 = t5351 * t17644;
    let t17646 = t3626 * t17645;
    let t17649 = t372 * t5296;
    (t17634, t17635, t17638, t17639, t17640, t17644, t17645, t17646, t17649)
}
