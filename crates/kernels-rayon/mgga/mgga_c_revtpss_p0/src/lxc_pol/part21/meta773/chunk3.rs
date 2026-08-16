//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2748/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2748(t1558: f64, t2645: f64, t10868: f64, t2482: f64, t814: f64, t14547: f64, t14671: f64, t14686: f64, t2661: f64, t2662: f64, t2754: f64, t4416: f64) -> (f64, f64, f64) {
    let t50560 = t1558 * t2645;
    let t50570 = t2482 * t10868 * t814;
    let t50573 = t50570 * t14686 * t14671 * t14547;
    let t50577 = t2661 * t2662 * t4416 * t2754;
    (t50560, t50573, t50577)
}
