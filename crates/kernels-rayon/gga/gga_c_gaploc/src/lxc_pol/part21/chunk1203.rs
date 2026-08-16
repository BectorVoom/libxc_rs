//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1203/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1203(t32328: f64, t2932: f64, t7064: f64, t7177: f64, t10698: f64, t1841: f64, t21476: f64, t7313: f64, t24321: f64, t2558: f64, t9647: f64, t1843: f64, t24478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32329 = 0.64087718584518535698e-3_f64 * t32328;
    let t32331 = t7064 * t2932 * t7177;
    let t32332 = 0.32043859292259267849e-3_f64 * t32331;
    let t32333 = t1841 * t10698;
    let t32334 = 0.25635087433807414279e-2_f64 * t32333;
    let t32336 = t21476 * t2932 * t7313;
    let t32337 = 0.64087718584518535698e-3_f64 * t32336;
    let t32339 = t9647 * t24321 * t2558;
    let t32340 = 0.32043859292259267849e-3_f64 * t32339;
    let t32342 = t7064 * t1843 * t24478;
    (t32329, t32332, t32334, t32337, t32340, t32342)
}
