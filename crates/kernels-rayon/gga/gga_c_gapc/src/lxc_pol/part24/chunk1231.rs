//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1231/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1231(t11234: f64, t11235: f64, t14891: f64, t11428: f64, t6: f64, t101: f64, t14875: f64, t14880: f64, t3940: f64, t5698: f64, t11181: f64, t11262: f64, t4865: f64) -> (f64, f64, f64, f64) {
    let t35422 = t11234 * t11235 * t14891;
    let t35424 = t6 * t11428;
    let t35429 = t35424 * t101 * t14875 * t3940 * t5698 * t14880;
    let t35432 = t11181 * t4865 * t11262;
    (t35422, t35424, t35429, t35432)
}
