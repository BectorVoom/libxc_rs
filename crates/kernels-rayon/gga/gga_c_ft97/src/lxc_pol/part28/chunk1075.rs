//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1075/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1075(t137198: f64, t137205: f64, t137213: f64, t137215: f64, t137219: f64, t137229: f64, t145667: f64, t145669: f64, t145673: f64, t145676: f64, t145681: f64, t145684: f64, t145687: f64, t145691: f64, t145695: f64, t145699: f64) -> f64 {
    let t145906 = t145667 / 3.0_f64 + t137198 + t137205 - t137213 + 4.0_f64 / 9.0_f64 * t145669 + 2.0_f64 / 27.0_f64 * t145673 - 8.0_f64 / 9.0_f64 * t145676 + t137215 / 9.0_f64 - t137219 - t137229 / 27.0_f64 - 4.0_f64 / 9.0_f64 * t145681 + 2.0_f64 / 3.0_f64 * t145684 - 2.0_f64 / 9.0_f64 * t145687 + 4.0_f64 / 3.0_f64 * t145691 + 4.0_f64 / 3.0_f64 * t145695 - 2.0_f64 * t145699;
    t145906
}
