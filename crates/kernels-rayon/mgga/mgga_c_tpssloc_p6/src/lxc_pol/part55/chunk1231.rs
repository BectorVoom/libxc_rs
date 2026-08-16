//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1231/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1231(t1992: f64, t22635: f64, t31090: f64, t5353: f64, t114160: f64, t6888: f64, t7691: f64, t26189: f64, t31137: f64, t31169: f64, t5234: f64, t31172: f64) -> (f64, f64, f64, f64) {
    let t120334 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t31090 * t5353;
    let t120337 = 0.3289868133696452873e-1_f64 * t6888 * t114160 * t7691;
    let t120340 = 0.3289868133696452873e-1_f64 * t6888 * t31137 * t26189;
    let t120341 = t5234 * t31169;
    let t120342 = t120341 * t31172;
    (t120334, t120337, t120340, t120342)
}
