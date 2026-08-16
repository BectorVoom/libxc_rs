//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1056/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1056(t31137: f64, t6907: f64, t1985: f64, t6891: f64, t6888: f64, t1323: f64, t8470: f64, t6920: f64, t8462: f64, t1307: f64, t1998: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31138 = t31137 * t6907;
    let t31140 = 0.16449340668482264365e-1_f64 * t1985 * t31138;
    let t31145 = t31137 * t6891;
    let t31147 = 0.3289868133696452873e-1_f64 * t6888 * t31145;
    let t31151 = t1323 * t8470;
    let t31153 = t6920 * t8462;
    let t31154 = 0.56521858531796547196e-2_f64 * t31153;
    let t31156 = t1998 * t59 * t1307;
    (t31138, t31140, t31145, t31147, t31151, t31154, t31156)
}
