//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1370/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1370(t1457: f64, t1572: f64, t31711: f64, t10463: f64, t4950: f64, t10477: f64, t17551: f64, t3384: f64, t204: f64, t2476: f64, t32033: f64, t123: f64, t7861: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34352 = 0.14300195980740170668e1_f64 * t1572 * t1457 * t31711;
    let t34354 = 0.14300195980740170668e1_f64 * t4950 * t10463;
    let t34356 = 0.14300195980740170668e1_f64 * t4950 * t10477;
    let t34358 = 0.71500979903700853338e0_f64 * t17551 * t3384;
    let t34361 = 0.18404604457881959845e2_f64 * t2476 * t204 * t32033;
    let t34363 = t7861 * t123 * t883;
    (t34352, t34354, t34356, t34358, t34361, t34363)
}
