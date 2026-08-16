//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 900/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk900(t26: f64, t326: f64, t35928: f64, t649: f64, t8941: f64, t69151: f64, t75770: f64, t69481: f64, t75773: f64, t69484: f64, t75779: f64, t74968: f64) -> (f64, f64, f64, f64, f64) {
    let t76159 = t326 * t35928 * t26 * t649 * t8941;
    let t76161 = t69151 * t75770;
    let t76163 = t69481 * t75773;
    let t76165 = t69484 * t75779;
    let t76167 = t69481 * t74968;
    (t76159, t76161, t76163, t76165, t76167)
}
