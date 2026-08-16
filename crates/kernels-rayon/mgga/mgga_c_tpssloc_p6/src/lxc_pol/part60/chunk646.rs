//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 646/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk646(t1893: f64, t8339: f64, t235: f64, t240: f64, t226: f64, t248: f64, t818: f64) -> (f64, f64, f64, f64) {
    let t8340 = t1893 * t8339;
    let t8342 = t235 * t240;
    let t8343 = t226 * t8342;
    let t8344 = t818 * t248;
    (t8340, t8342, t8343, t8344)
}
