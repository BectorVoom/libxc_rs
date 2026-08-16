//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 761/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk761(t1716: f64, t2541: f64, t1949: f64, t945: f64, t1933: f64, t78: f64, t278: f64, t481: f64) -> (f64, f64, f64) {
    let t7204 = t2541 * t1716;
    let t7207 = t1949 * t945;
    let t7209 = t78 * t1933;
    let t7211 = t481 * t7209 * t278;
    (t7204, t7207, t7211)
}
