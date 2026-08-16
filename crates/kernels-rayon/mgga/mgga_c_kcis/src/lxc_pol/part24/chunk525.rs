//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 525/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk525(t1008: f64, t4781: f64, t1014: f64, t1750: f64, t1126: f64, t1749: f64, t303: f64, t1800: f64, t922: f64, t3202: f64, t3200: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4782 = t4781 * t1008;
    let t4787 = t1014 * t1750;
    let t4789 = t1749 * t1126;
    let t4790 = t303 * t4789;
    let t4792 = t1800 * t922;
    let t4793 = t3202 * t4792;
    let t4794 = t3200 * t4793;
    let t4796 = t1804 * t922;
    (t4782, t4787, t4789, t4790, t4792, t4793, t4794, t4796)
}
