//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1280/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1280(t37556: f64, t37564: f64, t39097: f64, t39099: f64, t40515: f64, t42229: f64, t44915: f64, t44918: f64, t44921: f64, t44926: f64, t44928: f64, t44931: f64, t44933: f64, t44935: f64, t44937: f64) -> f64 {
    let t45015 = 0.60975299583150056624e-3_f64 * t40515 - t42229 - t44915 + t44918 + 0.162600798888400151e-2_f64 * t37556 + t39097 - t44921 - 0.30487649791575028312e-3_f64 * t37564 - t39099 + t44926 + t44928 + t44931 + t44933 - t44935 + t44937;
    t45015
}
