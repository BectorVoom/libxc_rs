//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 792/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk792(t2010: f64, t2415: f64, t7894: f64, t15039: f64, t2019: f64, t2020: f64, t2012: f64, t8817: f64, t13815: f64, t2339: f64, t7553: f64, t13819: f64, t8352: f64) -> (f64, f64, f64, f64, f64) {
    let t74256 = t2010 * t2415 * t7894;
    let t74259 = t2019 * t2020 * t15039;
    let t74262 = t2010 * t2012 * t8817;
    let t74267 = t7553 * t13815 * t2339;
    let t74269 = t13819 * t8352;
    (t74256, t74259, t74262, t74267, t74269)
}
