//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 519/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk519(t1487: f64, t4200: f64, t1486: f64, t469: f64, t382: f64, t41: f64, t3742: f64, t3783: f64, t484: f64, t3786: f64, t470: f64, t487: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4201 = t1487 * t4200;
    let t4203 = t1486 * t469;
    let t4204 = t41 * t382;
    let t4205 = t4204 * t3742;
    let t4206 = t4203 * t4205;
    let t4208 = t484 * t3783;
    let t4209 = t4208 * sigma0;
    let t4210 = t470 * t3786;
    let t4211 = t487 * t4210;
    (t4201, t4203, t4204, t4205, t4206, t4208, t4209, t4210, t4211)
}
