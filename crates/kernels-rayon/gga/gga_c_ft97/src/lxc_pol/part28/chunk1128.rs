//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1128/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1128(t139212: f64, t139352: f64, t148280: f64, t27081: f64, t32962: f64, t34822: f64, t379: f64, t23667: f64, t5899: f64, t32897: f64, t32899: f64, t3450: f64, t36571: f64, t637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148282 = t139212 * t139352 * t148280;
    let t148284 = t32962 * t27081;
    let t148286 = t139212 * t139352 * t148284;
    let t148288 = t34822 * t379;
    let t148290 = t5899 * t23667 * t148288;
    let t148295 = t32897 * t637 * t36571 * t32899 * t3450;
    (t148282, t148284, t148286, t148288, t148290, t148295)
}
