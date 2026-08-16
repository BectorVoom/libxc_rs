//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2810/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2810(t2723: f64, t2782: f64, t4503: f64, t51625: f64, t10661: f64, t14602: f64, t1558: f64, t2482: f64, t10535: f64, t14523: f64, t9285: f64, t10073: f64, t14496: f64) -> (f64, f64, f64, f64) {
    let t51628 = t2782 * t4503 * t51625 * t2723;
    let t51632 = t2482 * t10661 * t1558 * t14602;
    let t51635 = t10535 * t14523 * t9285;
    let t51637 = t10073 * t14496;
    (t51628, t51632, t51635, t51637)
}
