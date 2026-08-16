//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2586/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2586(t15382: f64, t3447: f64, t44525: f64, t11588: f64, t4928: f64, t3451: f64, t15357: f64, t3448: f64, t14740: f64, t15419: f64, t11584: f64, t15338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52022 = t3447 * t44525 * t15382;
    let t52036 = t11588 * t4928;
    let t52038 = t3447 * t52036 * t3451;
    let t52040 = t3448 * t15357;
    let t52050 = t3447 * t15419 * t14740;
    let t52053 = t3447 * t15338 * t11584;
    (t52022, t52036, t52038, t52040, t52050, t52053)
}
