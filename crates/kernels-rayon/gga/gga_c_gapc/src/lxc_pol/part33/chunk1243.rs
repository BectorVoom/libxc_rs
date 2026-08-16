//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1243/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1243(t3640: f64, t8341: f64, t11224: f64, t2933: f64, t1484: f64, t11203: f64, t8286: f64, t8297: f64, t11254: f64, t518: f64, t1460: f64, t3652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35386 = t8341 * t3640;
    let t35388 = t2933 * t11224;
    let t35390 = t1484 * t3640;
    let t35393 = t8286 * t11203 * t8297;
    let t35395 = t518 * t11254;
    let t35397 = t1460 * t3652;
    (t35386, t35388, t35390, t35393, t35395, t35397)
}
