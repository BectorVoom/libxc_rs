//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 763/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk763(t21719: f64, t35155: f64, t9183: f64, t236: f64, t446: f64, t615: f64, t14125: f64, t69009: f64, t14123: f64, t3116: f64, t3128: f64, t68575: f64, t8518: f64) -> (f64, f64, f64, f64) {
    let t73822 = t21719 * t35155 * t9183;
    let t73825 = t236 * t615 * t446;
    let t73827 = t69009 * t14125 * t73825;
    let t73833 = t3128 * t68575 * t3116 * t14123 * t14125 * t8518;
    (t73822, t73825, t73827, t73833)
}
