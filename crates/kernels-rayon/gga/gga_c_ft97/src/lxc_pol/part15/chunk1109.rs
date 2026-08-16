//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1109/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1109(t21624: f64, t3902: f64, t91: f64, t80691: f64, t992: f64, t2354: f64, t446: f64, t4934: f64, t4973: f64, t9770: f64, t4965: f64, t41879: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88218 = t91 * t3902 * t21624;
    let t88219 = t80691 * t992;
    let t88221 = t446 * t2354 * t88219;
    let t88223 = t4973 * t4934;
    let t88225 = t446 * t9770 * t88223;
    let t88227 = t4965 * t4934;
    let t88229 = t446 * t41879 * t88227;
    (t88218, t88219, t88221, t88223, t88225, t88227, t88229)
}
