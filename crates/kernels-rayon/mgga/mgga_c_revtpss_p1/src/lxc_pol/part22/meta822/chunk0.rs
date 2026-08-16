//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2937/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2937(t14202: f64, t9303: f64, t14238: f64, t2453: f64, t10142: f64, t10019: f64, t14239: f64, t1882: f64, t4066: f64, t1398: f64, t21990: f64, t10022: f64, t2782: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48005 = t9303 * t14202;
    let t48007 = t2453 * t14238;
    let t48008 = t48007 * t10142;
    let t48013 = t14239 * t10019;
    let t48015 = t4066 * t1882;
    let t48020 = t21990 * t1398;
    let t48022 = t2782 * t10022 * t48020;
    (t48005, t48007, t48008, t48013, t48015, t48020, t48022)
}
