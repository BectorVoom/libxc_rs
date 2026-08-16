//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 777/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk777(t15399: f64, t68764: f64, t21719: f64, t7248: f64, t9050: f64, t9054: f64, t9188: f64, t3352: f64, t9095: f64, t9137: f64, t15322: f64, t68432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74015 = t68764 * t15399;
    let t74018 = t21719 * t7248 * t9050;
    let t74021 = t21719 * t9188 * t9054;
    let t74024 = t21719 * t3352 * t9095;
    let t74027 = t21719 * t3352 * t9137;
    let t74033 = t68432 * t15322;
    (t74015, t74018, t74021, t74024, t74027, t74033)
}
