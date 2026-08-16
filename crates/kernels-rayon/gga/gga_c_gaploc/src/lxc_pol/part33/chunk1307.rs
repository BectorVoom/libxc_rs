//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1307/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1307(t1457: f64, t1572: f64, t31857: f64, t31711: f64, t10463: f64, t4950: f64, t10477: f64, t17551: f64, t3384: f64, t204: f64, t2476: f64, t32033: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34345 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t31857;
    let t34352 = 0.14300195980740170668e1_f64 * t1572 * t1457 * t31711;
    let t34354 = 0.14300195980740170668e1_f64 * t4950 * t10463;
    let t34356 = 0.14300195980740170668e1_f64 * t4950 * t10477;
    let t34358 = 0.71500979903700853338e0_f64 * t17551 * t3384;
    let t34361 = 0.18404604457881959845e2_f64 * t2476 * t204 * t32033;
    (t34345, t34352, t34354, t34356, t34358, t34361)
}
