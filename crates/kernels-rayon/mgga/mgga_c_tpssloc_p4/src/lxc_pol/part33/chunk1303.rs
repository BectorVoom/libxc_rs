//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1303/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1303(t23384: f64, t28660: f64, t28614: f64, t362: f64, t5914: f64, t28719: f64, t3216: f64, t112: f64, t28868: f64, t28904: f64, t576: f64, t580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100431 = t23384 * t28660;
    let t100436 = t23384 * t28614;
    let t100449 = t362 * t5914;
    let t100497 = t28719 * t3216;
    let t100911 = t28868 * t112;
    let t100945 = t576 * t28904;
    let t100946 = t28868 * t580;
    (t100431, t100436, t100449, t100497, t100911, t100945, t100946)
}
