//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2891/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2891(t60079: f64, t60158: f64, t60185: f64, t60214: f64, t60242: f64, t60279: f64, t60300: f64, t60329: f64, t17191: f64, t942: f64, t2929: f64, t5769: f64) -> (f64, f64, f64) {
    let t60332 = t60079 + t60158 + t60185 + t60214 + t60242 + t60279 + t60300 + t60329;
    let t60338 = t17191 * t942;
    let t60343 = t5769 * t2929;
    (t60332, t60338, t60343)
}
