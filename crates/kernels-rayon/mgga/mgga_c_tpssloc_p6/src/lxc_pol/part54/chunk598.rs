//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 598/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk598(t1055: f64, t4693: f64, t1052: f64, t1066: f64, t1635: f64, t3026: f64, t3169: f64, t388: f64, t4553: f64, t4555: f64, t4557: f64, t4559: f64, t4658: f64, t4660: f64, t4665: f64) -> f64 {
    let t4694 = t1055 * t4693;
    let t4696 = 2.0_f64 * t1052 * t4665 - t1052 * t4694 - t1066 * t4557 - t1066 * t4660 - t1635 * t3026 - t1635 * t3169 + t388 * t4553 + t388 * t4555 + t388 * t4559 + t388 * t4658;
    t4696
}
