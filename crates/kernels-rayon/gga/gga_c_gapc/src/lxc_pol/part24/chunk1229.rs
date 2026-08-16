//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1229/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1229(t11254: f64, t518: f64, t1460: f64, t3652: f64, t11258: f64, t3946: f64, t514: f64, t1005: f64, t13736: f64, t3639: f64, t4885: f64, t11273: f64, t8451: f64) -> (f64, f64, f64, f64, f64) {
    let t35395 = t518 * t11254;
    let t35397 = t1460 * t3652;
    let t35400 = t514 * t3946 * t11258;
    let t35404 = t1005 * t13736 * t3639 * t4885;
    let t35406 = t8451 * t11273;
    (t35395, t35397, t35400, t35404, t35406)
}
