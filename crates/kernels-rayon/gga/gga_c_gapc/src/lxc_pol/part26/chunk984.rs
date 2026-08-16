//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 984/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk984(t11933: f64, t311: f64, t3273: f64, t11499: f64, t325: f64, t2626: f64, t3374: f64) -> (f64, f64, f64) {
    let t11935 = t311 * t11933 * t3273;
    let t11937 = t325 * t11499;
    let t11938 = t2626 * t3374;
    (t11935, t11937, t11938)
}
