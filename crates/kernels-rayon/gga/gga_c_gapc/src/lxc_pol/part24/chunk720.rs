//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 720/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk720(t8545: f64, t996: f64, t2912: f64, t4538: f64, t2929: f64, t1599: f64, t2932: f64, t2958: f64, t1577: f64, t8399: f64, t2936: f64, t2937: f64, t4644: f64) -> (f64, f64, f64, f64, f64) {
    let t8546 = t996 * t8545;
    let t8547 = t8546 * t2912;
    let t8549 = t996 * t4538;
    let t8550 = t8549 * t2929;
    let t8552 = t2932 * t1599;
    let t8553 = t8552 * t2958;
    let t8556 = t8399 * t1577;
    let t8557 = t2936 * t8556;
    let t8559 = t2937 * t4644;
    (t8547, t8550, t8553, t8557, t8559)
}
