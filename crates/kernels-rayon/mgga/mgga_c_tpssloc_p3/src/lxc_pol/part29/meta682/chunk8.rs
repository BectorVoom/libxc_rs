//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2312/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2312(t2136: f64, t607: f64, t8027: f64, t1714: f64, t24682: f64, t460: f64, t27628: f64, t27634: f64, t10469: f64, t24719: f64, t3: f64, t86154: f64) -> (f64, f64, f64, f64, f64) {
    let t95370 = 0.16149102437656156342e-2_f64 * t8027 * t607 * t2136;
    let t95382 = t607 * t1714;
    let t95384 = t24682 * t95382 * t460;
    let t95387 = t27634 * t27628;
    let t95396 = t86154 * t3 * t24719 * t10469;
    (t95370, t95382, t95384, t95387, t95396)
}
