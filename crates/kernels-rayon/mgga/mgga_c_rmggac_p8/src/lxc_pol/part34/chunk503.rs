//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 503/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk503(t14084: f64, t7557: f64, t3052: f64, t4762: f64, t3051: f64, t1343: f64, t14082: f64) -> (f64, f64, f64, f64, f64) {
    let t14085 = t14084 * t7557;
    let t14088 = 1.0_f64 / t3052 / t4762;
    let t14089 = t3051 * t14088;
    let t14090 = t14082 * t1343;
    let t14091 = t14089 * t14090;
    (t14085, t14088, t14089, t14090, t14091)
}
