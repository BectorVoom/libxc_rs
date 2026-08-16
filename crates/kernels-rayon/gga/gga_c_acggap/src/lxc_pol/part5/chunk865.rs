//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 865/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk865(t1241: f64, t3243: f64, t158: f64, t2933: f64, t944: f64, t3045: f64, t3055: f64, t1210: f64, t939: f64, t3084: f64, t322: f64, t113: f64, t11805: f64, t11820: f64, t4: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12328 = t3243 * t1241;
    let t12331 = 1.0_f64 / t2933 / t158;
    let t12334 = t944 * t944;
    let t12344 = 0.15805078039045227836e2_f64 * t3055 * t3045;
    let t12345 = t939 * t1210;
    let t12349 = t3084 * t322;
    let t12357 = 0.43209876543209876543e0_f64 * t4 * t11805 * t113 + 0.27437962962962962965e0_f64 * t11820;
    (t12328, t12331, t12334, t12344, t12345, t12349, t12357)
}
