//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1079/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1079(t2106: f64, t4147: f64, t13625: f64, t531: f64, t7535: f64, t7238: f64, t2089: f64, t2371: f64, t198: f64, t206: f64, t2070: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26405 = t2106 * t4147;
    let t26406 = t26405 * t13625;
    let t26411 = t531 * t7535;
    let t26412 = t26411 * t7238;
    let t26415 = t2089 * t2371;
    let t26425 = t198 * t206 * t2070;
    (t26405, t26406, t26411, t26412, t26415, t26425)
}
