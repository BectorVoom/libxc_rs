//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 788/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk788(t103: f64, t4436: f64, t379: f64, t8217: f64, t16155: f64, t3194: f64, t8518: f64, t16160: f64, t8210: f64, t3193: f64, t432: f64, t4431: f64) -> (f64, f64, f64, f64) {
    let t16228 = t103 * t4436;
    let t16229 = t16228 * t379;
    let t16230 = t8217 * t16229;
    let t16233 = t3194 * t16155;
    let t16234 = t8518 * t16233;
    let t16237 = t8210 * t16160;
    let t16238 = t3193 * t16237;
    let t16241 = t4431 * t432;
    (t16230, t16234, t16238, t16241)
}
