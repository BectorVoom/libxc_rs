//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 992/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk992(t2132: f64, t30032: f64, t309: f64, t8336: f64, t2241: f64, t30005: f64, t2217: f64, t310: f64, t464: f64, t8357: f64, t96: f64, t104: f64, t8355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33320 = 0.15612530738769359031e2_f64 * t30032 * t2132 * t8336 * t309;
    let t33321 = t30005 * t2241;
    let t33323 = t310 * t2217;
    let t33324 = t33323 * t464;
    let t33333 = t96 * t8357;
    let t33335 = t104 * t8355;
    (t33320, t33321, t33323, t33324, t33333, t33335)
}
