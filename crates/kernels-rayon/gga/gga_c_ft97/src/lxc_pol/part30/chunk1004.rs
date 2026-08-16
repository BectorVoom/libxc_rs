//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1004/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1004(t10157: f64, t35353: f64, t446: f64, t713: f64, t35323: f64, t18: f64, t2354: f64, t3281: f64, t33476: f64, t150049: f64, t24432: f64, t24437: f64) -> (f64, f64, f64, f64) {
    let t150154 = t446 * t10157 * t35353 * t713;
    let t150158 = t446 * t10157 * t35323 * t713;
    let t150162 = t3281 * t2354 * t33476 * t18;
    let t150165 = t24437 * t24432 * t150049;
    (t150154, t150158, t150162, t150165)
}
