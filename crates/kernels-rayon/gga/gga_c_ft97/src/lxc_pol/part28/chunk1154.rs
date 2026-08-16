//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1154/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1154(t148403: f64, t5899: f64, t95344: f64, t148408: f64, t23667: f64, t148412: f64, t148417: f64, t95340: f64, t18: f64, t1969: f64, t3281: f64, t32979: f64) -> (f64, f64, f64, f64, f64) {
    let t148640 = t5899 * t95344 * t148403;
    let t148643 = t5899 * t23667 * t148408;
    let t148646 = t5899 * t23667 * t148412;
    let t148649 = t5899 * t95340 * t148417;
    let t148653 = t3281 * t1969 * t32979 * t18;
    (t148640, t148643, t148646, t148649, t148653)
}
