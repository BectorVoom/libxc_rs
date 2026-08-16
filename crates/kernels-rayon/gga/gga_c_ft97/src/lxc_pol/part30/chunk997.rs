//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 997/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk997(t33460: f64, t3875: f64, t6118: f64, t97078: f64, t6061: f64, t6135: f64, t992: f64, t24432: f64, t141314: f64, t3886: f64, t150034: f64, t97181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t150056 = t33460 * t3875;
    let t150058 = t6118 * t97078 * t150056;
    let t150060 = t6135 * t992 * t6061;
    let t150062 = t6118 * t24432 * t150060;
    let t150064 = t141314 * t3886;
    let t150066 = t6118 * t24432 * t150064;
    let t150069 = t6118 * t97181 * t150034;
    (t150056, t150058, t150060, t150062, t150064, t150066, t150069)
}
