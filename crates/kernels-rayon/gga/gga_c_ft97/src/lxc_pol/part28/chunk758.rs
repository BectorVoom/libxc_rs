//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 758/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk758(t32325: f64, t469: f64, t1317: f64, t28: f64, t375: f64, t7256: f64, t89: f64, t358: f64, t7165: f64) -> (f64, f64, f64, f64, f64) {
    let t32326 = t469 * t32325;
    let t32328 = t1317 * t28 * t32326;
    let t32331 = t89 * t375 * t7256;
    let t32332 = 2.0_f64 / 9.0_f64 * t32331;
    let t32333 = t7165 * t358;
    (t32326, t32328, t32331, t32332, t32333)
}
