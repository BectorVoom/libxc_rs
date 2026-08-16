//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 866/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk866(t86: f64, t34548: f64, t34790: f64, t113: f64, t5: f64, t7293: f64, t992: f64, t32967: f64, t6587: f64, t28: f64, t26567: f64, t5778: f64, t1039: f64, t7312: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87 = 10000000.0_f64 <= t86;
    let t34791 = t34548 + t34790;
    let t34798 = piecewise3(t87, 0.0_f64, t5 * t34791 * t113 / 4.0_f64 + t5 * t7293 * t992 / 4.0_f64);
    let t34799 = t32967 * t6587;
    let t34800 = t28 * t34799;
    let t34802 = t5778 * t26567;
    let t34803 = t28 * t34802;
    let t34808 = t7312 * t1039;
    (t34791, t34798, t34799, t34800, t34802, t34803, t34808)
}
