//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1255/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1255(t11234: f64, t35517: f64, t4018: f64, t11236: f64, t8570: f64, t11261: f64, t4868: f64, t11235: f64, t13537: f64, t13541: f64, t1577: f64, t8286: f64) -> (f64, f64, f64, f64, f64) {
    let t35519 = t11234 * t35517 * t4018;
    let t35521 = t8570 * t11236;
    let t35524 = t11261 * t35517 * t4868;
    let t35527 = t11261 * t11235 * t13537;
    let t35531 = t8286 * t13541 * t11235 * t1577;
    (t35519, t35521, t35524, t35527, t35531)
}
