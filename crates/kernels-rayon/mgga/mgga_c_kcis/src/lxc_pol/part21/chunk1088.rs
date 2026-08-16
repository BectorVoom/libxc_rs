//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1088/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1088(t26851: f64, t303: f64, t110: f64, t2174: f64, t2173: f64, t3049: f64, t3489: f64, t7687: f64, t7699: f64, t2175: f64, t26703: f64, t26823: f64, t26826: f64, t26829: f64, t26834: f64, t26837: f64, t26838: f64, t26841: f64, t26844: f64, t26846: f64, t26849: f64, t7703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26852 = t303 * t26851;
    let t26854 = t110 * t2174;
    let t26856 = 0.15445601851851851852e-3_f64 * t2173 * t26854;
    let t26857 = t3049 * t3489;
    let t26860 = t7687 * t7699;
    let t26864 = -0.69505208333333333333e-3_f64 * t26823 * t2175 + 0.33163888888888888888e-2_f64 * t26826 + 0.24872916666666666666e-2_f64 * t26829 + 0.24320185185185185185e-1_f64 * t26834 - t26837 - 0.88437037037037037034e-2_f64 * t26838 - 0.88437037037037037034e-2_f64 * t26841 + 0.16581944444444444444e-2_f64 * t26844 - 0.33163888888888888888e-2_f64 * t26846 + 0.33163888888888888888e-2_f64 * t26849 - 0.13265555555555555555e-1_f64 * t26852 + t26856 + 0.37069444444444444444e-2_f64 * t26857 * t2175 - 0.46336805555555555556e-3_f64 * t26860 + 0.46336805555555555556e-3_f64 * t7703 * t26703;
    (t26852, t26854, t26856, t26857, t26860, t26864)
}
