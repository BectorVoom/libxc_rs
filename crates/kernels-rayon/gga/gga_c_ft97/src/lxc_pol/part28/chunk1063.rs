//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1063/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1063(t1317: f64, t34397: f64, t376: f64, t136151: f64, t144893: f64, t32067: f64, t144857: f64, t2258: f64, t7243: f64, t136138: f64, t144813: f64, t34495: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t145667 = t1317 * t376 * t34397;
    let t145669 = t32067 * t136151 * t144893;
    let t145673 = t32067 * t2258 * t7243 * t144857;
    let t145676 = t32067 * t136138 * t144813;
    let t145681 = t89 * t376 * t34495;
    (t145667, t145669, t145673, t145676, t145681)
}
