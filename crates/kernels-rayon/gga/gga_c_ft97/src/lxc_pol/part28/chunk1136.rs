//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1136/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1136(t23925: f64, t28: f64, t6615: f64, t89: f64, t2185: f64, t27157: f64, t27158: f64, t32924: f64, t32962: f64, t3424: f64, t139431: f64, t32897: f64) -> (f64, f64, f64, f64) {
    let t148396 = t89 * t28 * t23925 * t6615;
    let t148401 = t27157 * t2185 * t32924 * t27158;
    let t148403 = t32962 * t3424;
    let t148405 = t32897 * t139431 * t148403;
    (t148396, t148401, t148403, t148405)
}
