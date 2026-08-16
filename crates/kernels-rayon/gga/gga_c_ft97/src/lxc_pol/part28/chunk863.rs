//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 863/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk863(t452: f64, t7211: f64, t986: f64, t110: f64, t34482: f64, t34514: f64, t83: f64, t7281: f64, t942: f64, t488: f64, t34569: f64, t34542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34758 = t452 * t986 * t7211;
    let t34762 = t452 * t110 * t34482;
    let t34765 = t83 * t34514;
    let t34768 = t7281 * t942;
    let t34770 = t452 * t488 * t34768;
    let t34773 = t83 * t34569;
    let t34776 = t83 * t34542;
    (t34758, t34762, t34765, t34768, t34770, t34773, t34776)
}
