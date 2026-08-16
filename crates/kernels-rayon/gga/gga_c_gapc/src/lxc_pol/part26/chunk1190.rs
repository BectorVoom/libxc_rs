//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1190/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1190(t11449: f64, t11452: f64, t190: f64, t424: f64, t11519: f64, t34656: f64, t11597: f64, t9304: f64, t9308: f64, t20768: f64, t34363: f64, t11495: f64, t1717: f64) -> (f64, f64, f64, f64, f64) {
    let t34739 = t424 * t190 * t11449 * t11452;
    let t34742 = t34656 * t11519;
    let t34745 = t9304 * t11597 * t9308;
    let t34747 = t34363 * t20768;
    let t34749 = t11495 * t1717;
    (t34739, t34742, t34745, t34747, t34749)
}
