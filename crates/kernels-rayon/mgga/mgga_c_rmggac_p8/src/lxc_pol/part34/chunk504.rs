//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 504/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk504(t14091: f64, t7557: f64, t3069: f64, t7494: f64, t2186: f64, t3154: f64, t2044: f64, t7554: f64, t2139: f64, t3157: f64, t3807: f64, t2048: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14092 = t14091 * t7557;
    let t14094 = t7494 * t3069;
    let t14100 = t2186 * t3154;
    let t14102 = t2044 * t7554;
    let t14103 = t2139 * t14102;
    let t14105 = t3807 * t3157;
    let t14107 = t236 * t2048;
    (t14092, t14094, t14100, t14102, t14103, t14105, t14107)
}
