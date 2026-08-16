//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1014/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1014(t150228: f64, t33282: f64, t7512: f64, t7515: f64, t35541: f64, t375: f64, t89: f64, t150233: f64, t7511: f64, t35525: f64, t681: f64, t150238: f64, t33301: f64) -> (f64, f64, f64, f64, f64) {
    let t150288 = t33282 * t7512 * t7515 * t150228;
    let t150291 = t89 * t375 * t35541;
    let t150295 = t7511 * t7512 * t7515 * t150233;
    let t150298 = t89 * t681 * t35525;
    let t150302 = t7511 * t7512 * t33301 * t150238;
    (t150288, t150291, t150295, t150298, t150302)
}
