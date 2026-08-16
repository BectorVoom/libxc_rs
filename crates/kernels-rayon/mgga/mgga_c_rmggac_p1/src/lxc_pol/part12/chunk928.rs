//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 928/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk928(t30900: f64, t35972: f64, t739: f64, t36292: f64, t5888: f64, t118: f64, t2001: f64, t2281: f64, t495: f64, t7717: f64, t2144: f64, t3351: f64, t352: f64, t7231: f64, t9104: f64) -> (f64, f64, f64, f64) {
    let t39994 = t739 * t35972 * t30900;
    let t39997 = t739 * t36292 * t5888;
    let t39998 = 0.15965655602485078085e0_f64 * t39997;
    let t40001 = t2001 * t118 * t2281 * t495;
    let t40002 = t7717 * t40001;
    let t40007 = t3351 * t7231 * t2144 * t9104 * t352;
    (t39994, t39998, t40002, t40007)
}
