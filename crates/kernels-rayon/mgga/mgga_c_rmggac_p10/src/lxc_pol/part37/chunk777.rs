//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 777/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk777(t14180: f64, t39277: f64, t12012: f64, t69045: f64, t11729: f64, t69041: f64, t11732: f64, t3046: f64, t3924: f64, t507: f64, t12140: f64, t69788: f64) -> (f64, f64, f64, f64, f64) {
    let t74049 = 0.1064114997332445985e-4_f64 * t39277 * t14180;
    let t74050 = t69045 * t12012;
    let t74052 = t69041 * t11729;
    let t74056 = t507 * t3924 * t3046 * t11732;
    let t74058 = t69788 * t12140;
    (t74049, t74050, t74052, t74056, t74058)
}
