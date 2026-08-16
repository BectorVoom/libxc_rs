//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1755/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1755(t6442: f64, t43946: f64, t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64) -> (f64, f64, f64) {
    let t90422 = t6442 * t6442;
    let t90423 = t43946 * t90422;
    let t90437 = 20.0_f64 / 9.0_f64 * t89824 - 8.0_f64 * t89828 - 80.0_f64 / 81.0_f64 * t89832 + 8.0_f64 / 9.0_f64 * t81156 - 8.0_f64 / 3.0_f64 * t81158 + 8.0_f64 / 9.0_f64 * t68255 - 2.0_f64 / 3.0_f64 * t89839 - 8.0_f64 / 9.0_f64 * t89843 + 12.0_f64 * t89847 + 2.0_f64 * t89851 + 8.0_f64 / 3.0_f64 * t89855;
    (t90422, t90423, t90437)
}
