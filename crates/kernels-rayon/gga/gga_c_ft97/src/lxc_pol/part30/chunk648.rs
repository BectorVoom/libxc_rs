//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 648/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk648(t2574: f64, t265: f64, t27841: f64, t242: f64, t27913: f64, t3842: f64, t6154: f64, t729: f64, t27897: f64, t1882: f64, t6858: f64, t6875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28438 = t2574 * t265 * t27841;
    let t28441 = t242 * t27913;
    let t28445 = t729 * t6154 * t3842;
    let t28448 = t242 * t27897;
    let t28451 = t1882 * t6858;
    let t28453 = t1882 * t6875;
    (t28438, t28441, t28445, t28448, t28451, t28453)
}
