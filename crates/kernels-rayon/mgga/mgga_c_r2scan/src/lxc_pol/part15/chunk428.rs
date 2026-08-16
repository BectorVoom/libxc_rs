//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 428/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk428(t424: f64, t725: f64, t41: f64, t410: f64, t661: f64, t1473: f64, t1474: f64, t1475: f64, t1476: f64, t1714: f64, t1717: f64) -> (f64, f64, f64, f64) {
    let t1793 = t424 * t725;
    let t1794 = t41 * t1793;
    let t1796 = t410 * t661;
    let t1800 = -0.21099166666666666667e0_f64 * t1714 + 0.16879333333333333333e1_f64 * t1717 + t1473 + t1474 + t1475 + t1476;
    (t1793, t1794, t1796, t1800)
}
