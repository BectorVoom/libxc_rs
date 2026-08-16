//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1880/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1880(t2035: f64, t27833: f64, t7313: f64, t7898: f64, t1032: f64, t1892: f64, t1955: f64) -> (f64, f64, f64, f64) {
    let t27834 = t27833 * t2035;
    let t27835 = t7898 * t7313;
    let t27836 = t1892 * t1032;
    let t27837 = t1955 * t27836;
    (t27834, t27835, t27836, t27837)
}
