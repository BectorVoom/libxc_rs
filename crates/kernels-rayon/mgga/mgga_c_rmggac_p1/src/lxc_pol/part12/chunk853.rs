//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 853/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk853(t1971: f64, t2144: f64, t495: f64, t5267: f64, t7230: f64, t3351: f64, t498: f64, t7231: f64, t3352: f64, t5145: f64, t5268: f64, t7262: f64) -> (f64, f64, f64, f64) {
    let t38913 = t7230 * t1971 * t2144 * t5267 * t495;
    let t38918 = t3351 * t7231 * t2144 * t5267 * t498;
    let t38922 = t3351 * t3352 * t2144 * t5145;
    let t38926 = t3351 * t1971 * t7262 * t5268;
    (t38913, t38918, t38922, t38926)
}
