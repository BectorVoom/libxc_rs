//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1012/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1012(t24543: f64, t35333: f64, t140756: f64, t140757: f64, t27878: f64, t33292: f64, t27819: f64, t27820: f64, t33341: f64, t729: f64, t24437: f64, t2574: f64, t27796: f64) -> (f64, f64, f64, f64) {
    let t150259 = t24543 * t35333;
    let t150263 = t33292 * t140756 * t140757 * t27878;
    let t150267 = t27819 * t729 * t33341 * t27820;
    let t150271 = t24437 * t2574 * t33341 * t27796;
    (t150259, t150263, t150267, t150271)
}
