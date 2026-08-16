//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1257/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1257(t10924: f64, t5679: f64, t6096: f64, t11069: f64, t5669: f64, t20671: f64, t25070: f64, t28856: f64, t11029: f64, t2087: f64, t4614: f64, t10951: f64, t5782: f64) -> (f64, f64, f64, f64, f64) {
    let t33269 = 0.71500979903700853338e0_f64 * t5679 * t10924 * t6096;
    let t33271 = 0.2044956050875773316e1_f64 * t5669 * t11069;
    let t33273 = t28856 * t20671 * t25070;
    let t33274 = 0.2556195063594716645e0_f64 * t33273;
    let t33282 = 0.18404604457881959845e2_f64 * t2087 * t4614 * t11029;
    let t33284 = 0.18404604457881959845e2_f64 * t5782 * t10951;
    (t33269, t33271, t33274, t33282, t33284)
}
