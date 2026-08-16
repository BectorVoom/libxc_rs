//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 511/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk511(t5147: f64, t762: f64, t242: f64, t1168: f64, t3977: f64, t1131: f64, t1175: f64, t729: f64, t265: f64, t5053: f64, t992: f64, t2600: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5148 = t762 * t5147;
    let t5149 = t242 * t5148;
    let t5152 = t3977 * t1168;
    let t5153 = t242 * t5152;
    let t5157 = t729 * t1175 * t1131;
    let t5161 = t729 * t265 * t5053;
    let t5165 = t992 * t1131;
    let t5166 = t2600 * t5165;
    (t5148, t5149, t5152, t5153, t5157, t5161, t5166)
}
