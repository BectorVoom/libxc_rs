//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1047/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1047(t18: f64, t3281: f64, t33460: f64, t9770: f64, t35516: f64, t668: f64, t2354: f64, t446: f64, t505: f64, t150064: f64, t150034: f64, t41879: f64) -> (f64, f64, f64, f64) {
    let t150966 = t3281 * t9770 * t33460 * t18;
    let t150968 = t35516 * t668;
    let t150971 = t446 * t2354 * t150968 * t505;
    let t150974 = t446 * t9770 * t150064;
    let t150977 = t446 * t41879 * t150034;
    (t150966, t150971, t150974, t150977)
}
