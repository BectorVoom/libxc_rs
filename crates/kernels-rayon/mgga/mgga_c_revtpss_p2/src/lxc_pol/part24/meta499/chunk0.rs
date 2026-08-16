//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1501/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1501(t14598: f64, t23160: f64, t686: f64, t72: f64, t23244: f64, t251: f64, t1568: f64, t5977: f64, t2723: f64, t2782: f64, t4503: f64, t1558: f64, t6041: f64) -> (f64, f64, f64, f64, f64) {
    let t76125 = t14598 * t23160 * t72 * t686;
    let t76127 = t251 * t23244;
    let t76131 = t1568 * t5977;
    let t76134 = t2782 * t4503 * t76131 * t2723;
    let t76136 = t6041 * t1558;
    (t76125, t76127, t76131, t76134, t76136)
}
