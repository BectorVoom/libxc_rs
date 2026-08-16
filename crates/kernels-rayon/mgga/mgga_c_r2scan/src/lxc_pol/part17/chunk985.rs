//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 985/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk985(t12098: f64, t3275: f64, t3277: f64, t11531: f64, t3472: f64, t10937: f64, t10952: f64, t10960: f64, t11364: f64, t11365: f64, t11367: f64, t11368: f64, t11372: f64, t11374: f64, t11375: f64, t11377: f64, t12093: f64, t12096: f64) -> (f64, f64, f64) {
    let t12100 = t3275 * t12098 * t3277;
    let t12101 = 5.0_f64 / 16.0_f64 * t12100;
    let t12103 = t3275 * t3472 * t11531;
    let t12104 = 5.0_f64 / 16.0_f64 * t12103;
    let t12107 = t12093 + t12096 + t11364 - t11365 + 0.1921128438866447784e-2_f64 * t10937 + t12101 + t12104 + t11367 + t11368 + 0.43368970657079495308e-4_f64 * t10952 + t11372 - 0.30487649791575028312e-3_f64 * t10960 - t11374 + t11375 + t11377;
    (t12100, t12103, t12107)
}
