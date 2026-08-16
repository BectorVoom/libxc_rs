//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 906/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk906(t11257: f64, t11258: f64, t3650: f64, t4865: f64, t11235: f64, t4868: f64, t2922: f64, t3646: f64, t3643: f64, t8492: f64, t3694: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11259 = t11257 * t11258;
    let t11261 = t3650 * t4865;
    let t11262 = t11235 * t4868;
    let t11263 = t11261 * t11262;
    let t11265 = t2922 * t3646;
    let t11267 = t3643 * t8492;
    let t11268 = t11267 * t3646;
    let t11270 = t5 * t3694;
    (t11259, t11261, t11262, t11263, t11265, t11268, t11270)
}
