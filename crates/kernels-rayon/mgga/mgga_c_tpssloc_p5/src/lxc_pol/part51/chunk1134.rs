//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1134/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1134(t12524: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t20173: f64, t2039: f64, t24462: f64, t24465: f64, t27170: f64, t27240: f64, t27254: f64, t27273: f64, t27276: f64, t27281: f64, t3938: f64, t3941: f64, t4072: f64, t5371: f64, t5376: f64, t577: f64, t671: f64, t7056: f64, t7230: f64, t7235: f64, t7801: f64, t7956: f64) -> f64 {
    let t27286 = 0.45e1_f64 * t27240 * t577 + 0.135e2_f64 * t27254 * t671 + 0.135e2_f64 * t24462 * t1458 + 27.0_f64 * t24465 * t5376 + 0.135e2_f64 * t7230 * t4072 + 0.135e2_f64 * t16521 * t2039 + 27.0_f64 * t16524 * t7235 + 0.135e2_f64 * t5371 * t7056 + 27.0_f64 * t12524 * t7956 + 27.0_f64 * t20173 * t7956 + 27.0_f64 * t3941 * t27273 + 27.0_f64 * t3941 * t27276 + 0.135e2_f64 * t3938 * t7801 + 27.0_f64 * t3941 * t27281 + 0.135e2_f64 * t1401 * t27170;
    t27286
}
