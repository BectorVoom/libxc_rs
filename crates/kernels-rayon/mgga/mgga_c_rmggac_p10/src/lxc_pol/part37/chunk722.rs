//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 722/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk722(t70320: f64, t212: f64, t28: f64, t3144: f64, t4071: f64, t672: f64, t14015: f64, t14371: f64, t16059: f64, t511: f64, t7231: f64, t2046: f64, t2049: f64, t2169: f64) -> (f64, f64, f64, f64, f64) {
    let t70321 = 0.49700494569958178262e-1_f64 * t70320;
    let t70328 = t672 * t212 * t4071 * t28 * t3144;
    let t70330 = t14371 * t14015;
    let t70336 = t511 * t16059;
    let t70337 = t7231 * t70336;
    let t70358 = t2046 * t2049 * t2169;
    (t70321, t70328, t70330, t70337, t70358)
}
