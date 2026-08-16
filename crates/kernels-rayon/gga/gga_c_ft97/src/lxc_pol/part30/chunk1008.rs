//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1008/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1008(t141319: f64, t3886: f64, t2354: f64, t446: f64, t140790: f64, t9744: f64, t140756: f64, t140762: f64, t27814: f64, t33294: f64, t10157: f64, t33341: f64, t3837: f64, t6118: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150202 = t141319 * t3886;
    let t150204 = t446 * t2354 * t150202;
    let t150206 = t140790 * t3886;
    let t150208 = t446 * t9744 * t150206;
    let t150212 = t140762 * t140756 * t33294 * t27814;
    let t150216 = t6118 * t10157 * t33341 * t3837;
    (t150202, t150204, t150206, t150208, t150212, t150216)
}
