//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1033/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1033(t140920: f64, t3729: f64, t140919: f64, t2393: f64, t3733: f64, t1609: f64, t218: f64, t2378: f64, t140943: f64, t35426: f64, t35427: f64, t1109: f64, t17839: f64) -> (f64, f64, f64, f64, f64) {
    let t150727 = t140920 * t3729;
    let t150731 = t140919 * t2393 * t3733;
    let t150736 = t1609 * t218 * t2378 * t3733;
    let t150740 = t35426 * t140943 * t35427;
    let t150751 = t17839 * t1109;
    (t150727, t150731, t150736, t150740, t150751)
}
