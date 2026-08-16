//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 379/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk379(t6045: f64, t6250: f64, t1472: f64, t6051: f64, t4113: f64, t6241: f64) -> (f64, f64, f64) {
    let t6251 = t6045 * t6250;
    let t6255 = 0.16669500273148148149e-1_f64 * t1472 * t6051;
    let t6256 = t4113 * t6241;
    (t6251, t6255, t6256)
}
