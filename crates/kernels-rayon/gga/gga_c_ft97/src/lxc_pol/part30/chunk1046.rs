//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1046/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1046(t140757: f64, t140762: f64, t140833: f64, t27850: f64, t27820: f64, t33294: f64, t631: f64, t97168: f64, t1434: f64, t150912: f64, t193: f64, t743: f64) -> (f64, f64, f64) {
    let t150953 = t140762 * t140833 * t140757 * t27850;
    let t150958 = t97168 * t631 * t140833 * t33294 * t27820;
    let t150962 = t1434 * t193 * t743 * t150912;
    (t150953, t150958, t150962)
}
