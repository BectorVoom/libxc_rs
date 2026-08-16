//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1188/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1188(t18728: f64, t18731: f64, t18734: f64, t18747: f64, t18749: f64, t1282: f64, t18718: f64, t18721: f64, t18725: f64, t18745: f64, t18752: f64, t342: f64, t63: f64, t7306: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21461 = 1.9486833333333333_f64 * t18728;
    let t21462 = 1.4615125_f64 * t18731;
    let t21463 = 0.9743416666666667_f64 * t18734;
    let t21465 = 3.8973666666666666_f64 * t18747;
    let t21466 = 1.9486833333333333_f64 * t18749;
    let t21468 = 5.87616_f64 * t63 * t1282 * t7306 * t342 + 5.87616_f64 * t18718 + 2.20356_f64 * t18721 - 1.46904_f64 * t18725 - t21461 + t21462 + t21463 - 2.0_f64 / 3.0_f64 * t18745 + t21465 - t21466 + t18752 / 2.0_f64;
    (t21461, t21462, t21463, t21465, t21466, t21468)
}
