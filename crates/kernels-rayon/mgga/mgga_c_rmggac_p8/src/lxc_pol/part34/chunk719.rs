//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 719/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk719(t2044: f64, t333: f64, t7273: f64, t7554: f64, t14362: f64, t1993: f64, t13868: f64, t13797: f64, t14077: f64, t7282: f64, t1986: f64, t2090: f64) -> (f64, f64, f64, f64, f64) {
    let t70198 = t7273 * t2044 * t7554 * t333;
    let t70207 = t1993 * t14362;
    let t70208 = t70207 * t13868;
    let t70211 = t7282 * t14077 * t13797;
    let t70212 = 0.10909864661698136691e0_f64 * t70211;
    let t70221 = t1986 * t2090;
    (t70198, t70207, t70208, t70212, t70221)
}
