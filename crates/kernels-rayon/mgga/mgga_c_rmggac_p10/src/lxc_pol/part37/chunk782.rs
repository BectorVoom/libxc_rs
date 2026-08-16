//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 782/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk782(t21713: f64, t74120: f64, t9189: f64, t21714: f64, t9193: f64, t9197: f64, t14125: f64, t68421: f64, t73699: f64, t14124: f64, t236: f64, t498: f64, t598: f64, t68422: f64) -> (f64, f64, f64, f64, f64) {
    let t74122 = t21713 * t74120 * t9189;
    let t74125 = t21713 * t21714 * t9193;
    let t74128 = t21713 * t21714 * t9197;
    let t74131 = t68421 * t14125 * t73699;
    let t74137 = t14124 * t68422 * t236 * t598 * t498;
    (t74122, t74125, t74128, t74131, t74137)
}
