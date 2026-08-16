//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1143/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1143(t147647: f64, t446: f64, t9073: f64, t34817: f64, t40830: f64, t558: f64, t34827: f64, t376: f64, t5890: f64, t1369: f64, t34835: f64, t147122: f64, t28: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t148492 = t446 * t9073 * t147647;
    let t148496 = t446 * t40830 * t34817 * t558;
    let t148499 = t5890 * t376 * t34827;
    let t148502 = t1369 * t376 * t34835;
    let t148508 = t89 * t28 * t147122 * t558;
    (t148492, t148496, t148499, t148502, t148508)
}
