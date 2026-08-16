//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1617/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1617(t39442: f64, t39483: f64, t39520: f64, t87303: f64, t87304: f64, t87305: f64, t87306: f64, t87307: f64, t87309: f64, t87312: f64, t87314: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t87315: f64, t87318: f64) -> (f64, f64) {
    let t87635 = t39442 + t87303 + t87304 + t87305 + t87306 - t87307 + t87309 + t87312 - t39483 + t39520 + t87314;
    let t87637 = -t39528 + t39531 + t87315 + t39534 + t39537 - t39540 + t39741 + t39744 + t39747 + t87318 + t39750;
    (t87635, t87637)
}
