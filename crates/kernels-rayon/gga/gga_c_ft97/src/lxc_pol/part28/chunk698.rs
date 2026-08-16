//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 698/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk698(t26768: f64, t526: f64, t27: f64, t89: f64, t23667: f64, t27096: f64, t5899: f64, t590: f64, t6615: f64, t586: f64, t28: f64, t5890: f64) -> (f64, f64, f64, f64, f64) {
    let t27114 = t526 * t26768;
    let t27116 = t89 * t27 * t27114;
    let t27120 = t23667 * t27096;
    let t27121 = t5899 * t27120;
    let t27123 = t6615 * t590;
    let t27124 = t586 * t27123;
    let t27126 = t5890 * t28 * t27124;
    (t27114, t27116, t27121, t27123, t27126)
}
