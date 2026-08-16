//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 174/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk174(t1017: f64, t586: f64, t24: f64, t1033: f64, t462: f64, t581: f64, t92: f64) -> (f64, f64) {
    let t1036 = t586 * t1017;
    let t1037 = t24 * t1036;
    let t1039 = -t581 - t462 * t1033 / 3.0_f64 - t92 * t1037;
    (t1037, t1039)
}
