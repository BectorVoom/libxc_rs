//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 891/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk891(t17766: f64, t2594: f64, t446: f64, t5053: f64, t668: f64, t505: f64, t2354: f64, t4934: f64, t9770: f64, t18: f64, t3699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17767 = t2594 * t17766;
    let t17768 = t446 * t17767;
    let t17770 = t5053 * t668;
    let t17771 = t17770 * t505;
    let t17772 = t2354 * t17771;
    let t17773 = t446 * t17772;
    let t17775 = t4934 * t668;
    let t17776 = t17775 * t505;
    let t17777 = t9770 * t17776;
    let t17778 = t446 * t17777;
    let t17780 = t3699 * t18;
    (t17768, t17771, t17773, t17776, t17778, t17780)
}
