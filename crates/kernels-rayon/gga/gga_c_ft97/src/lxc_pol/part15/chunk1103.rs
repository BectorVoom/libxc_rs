//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1103/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1103(t4635: f64, t4917: f64, t4934: f64, t18391: f64, t5147: f64, t1131: f64, t21351: f64, t41816: f64, t446: f64, t21369: f64, t2354: f64, t79697: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t88105 = t4917 * t4635;
    let t88114 = t4917 * t4934;
    let t88131 = t18391 * t5147;
    let t88141 = t21351 * t1131;
    let t88143 = t446 * t41816 * t88141;
    let t88145 = t21369 * t1131;
    let t88147 = t446 * t2354 * t88145;
    let t88149 = t79697 * t992;
    (t88105, t88114, t88131, t88141, t88143, t88145, t88147, t88149)
}
