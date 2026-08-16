//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1350/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1350(t10318: f64, t4360: f64, t4667: f64, t10319: f64, t4753: f64, t2413: f64, t26122: f64, t26726: f64, t901: f64, t26822: f64, t10315: f64, t20445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35136 = 0.71500979903700853338e0_f64 * t4360 * t10318 * t4667;
    let t35138 = 0.47667319935800568892e0_f64 * t10319 * t4753;
    let t35140 = 0.21450293971110256002e1_f64 * t26122 * t2413;
    let t35141 = t26726 * t901;
    let t35142 = 0.29792074959875355558e-1_f64 * t35141;
    let t35143 = t26822 * t901;
    let t35144 = 0.14896037479937677779e-1_f64 * t35143;
    let t35146 = 0.14300195980740170668e1_f64 * t20445 * t10315;
    (t35136, t35138, t35140, t35142, t35144, t35146)
}
