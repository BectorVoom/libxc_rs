//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2620/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2620(t15908: f64, t9882: f64, t118: f64, t2375: f64, t5151: f64, t16169: f64, t2663: f64, t15892: f64, t2371: f64, t5154: f64, t9919: f64, t5173: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53779 = t15908 * t9882;
    let t53782 = t5151 * t118 * t2375;
    let t53787 = t16169 * t2663;
    let t53796 = t15892 * t2371;
    let t53798 = t5154 * t9919;
    let t53825 = 16.0_f64 * t5173 * t591;
    (t53779, t53782, t53787, t53796, t53798, t53825)
}
