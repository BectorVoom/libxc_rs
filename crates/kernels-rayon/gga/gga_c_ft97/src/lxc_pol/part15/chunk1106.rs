//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1106/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1106(t4934: f64, t4969: f64, t446: f64, t9770: f64, t1131: f64, t21362: f64, t2354: f64, t4973: f64, t5053: f64, t17765: f64, t4635: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88169 = t4969 * t4934;
    let t88171 = t446 * t9770 * t88169;
    let t88176 = t21362 * t1131;
    let t88178 = t446 * t2354 * t88176;
    let t88180 = t4973 * t5053;
    let t88182 = t446 * t2354 * t88180;
    let t88184 = t17765 * t4635;
    (t88169, t88171, t88176, t88178, t88180, t88182, t88184)
}
