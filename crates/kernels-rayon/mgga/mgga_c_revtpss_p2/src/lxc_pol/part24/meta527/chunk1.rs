//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1561/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1561(t5390: f64, t6601: f64, t21177: f64, t5362: f64, t1235: f64, t127: f64, t24634: f64, t371: f64, t20842: f64, t5327: f64, t17396: f64, t20926: f64) -> (f64, f64, f64, f64, f64) {
    let t83728 = t6601 * t5390;
    let t83731 = t21177 * t5362;
    let t83735 = t1235 * t371 * t127 * t24634;
    let t83748 = t5327 * t20842;
    let t83751 = t17396 * t20926;
    (t83728, t83731, t83735, t83748, t83751)
}
