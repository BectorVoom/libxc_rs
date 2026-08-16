//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 435/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk435(t167: f64, t2185: f64, t6630: f64, t1060: f64, t1359: f64, t574: f64, t1053: f64) -> (f64, f64, f64) {
    let t6632 = t2185 * t167 * t6630;
    let t6636 = t574 * t1060 * t1359;
    let t6639 = t1359 * t1053;
    (t6632, t6636, t6639)
}
