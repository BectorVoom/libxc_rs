//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 899/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk899(t15078: f64, t9128: f64, t14173: f64, t3928: f64, t5267: f64, t1550: f64, t5888: f64, t69184: f64, t1635: f64, t26144: f64, t3065: f64, t5898: f64) -> (f64, f64, f64, f64, f64) {
    let t76110 = t9128 * t15078;
    let t76113 = t3928 * t14173 * t5267;
    let t76116 = t1550 * t69184 * t5888;
    let t76119 = t26144 * t3065 * t1635;
    let t76122 = t3928 * t14173 * t5898;
    (t76110, t76113, t76116, t76119, t76122)
}
