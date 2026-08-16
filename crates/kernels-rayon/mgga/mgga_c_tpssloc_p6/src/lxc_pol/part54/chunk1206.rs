//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1206/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1206(t32834: f64, t6591: f64, t1510: f64, t6612: f64, t6605: f64, t1499: f64, t8342: f64, t8344: f64, t232: f64, t4180: f64, t4181: f64, t30714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32835 = t6591 * t32834;
    let t32837 = t6612 * t1510;
    let t32838 = t6605 * t32837;
    let t32840 = t1499 * t8342;
    let t32841 = t32840 * t8344;
    let t32844 = t4180 * t4181 * t232;
    let t32845 = t30714 * t32844;
    (t32835, t32837, t32838, t32840, t32841, t32844, t32845)
}
