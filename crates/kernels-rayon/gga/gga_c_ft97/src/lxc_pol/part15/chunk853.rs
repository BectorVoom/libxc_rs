//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 853/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk853(t240: f64, t7513: f64, t294: f64, t7639: f64, t1107: f64, t5011: f64, t13: f64, t21: f64, t2: f64, t7242: f64, t113: f64, t10: f64, t11175: f64, t83: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33300 = 1.0_f64 / t7513 / t240;
    let t33828 = 1.0_f64 / t7639 / t294;
    let t35382 = t5011 * t1107;
    let t36377 = t13 * t21;
    let t36452 = t7242 * t2;
    let t36827 = t13 * t113;
    let t37292 = t10 * t11175 * t83;
    (t33300, t33828, t35382, t36377, t36452, t36827, t37292)
}
