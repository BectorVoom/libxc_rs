//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 718/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk718(t14362: f64, t2190: f64, t3144: f64, t25561: f64, t29: f64, t3117: f64, t3132: f64, t3136: f64, t13839: f64, t2044: f64, t352: f64, t7554: f64) -> (f64, f64, f64, f64) {
    let t70176 = t2190 * t14362 * t3144;
    let t70186 = t3117 * t25561 * t29;
    let t70188 = t3132 * t70186 * t3136;
    let t70194 = t13839 * t2044 * t7554 * t352;
    (t70176, t70186, t70188, t70194)
}
