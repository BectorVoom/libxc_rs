//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1235/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1235(t11013: f64, t225: f64, t10163: f64, t386: f64, t68: f64, t11008: f64, t3215: f64, t112: f64, t12512: f64, t111: f64, t3931: f64, t2311: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43599 = t11013 * t225;
    let t43603 = 1.0_f64 / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43619 = t11008 * t225;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0_f64 / t43636;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45602 = t2311 * t671;
    (t43599, t43604, t43619, t43637, t45557, t45560, t45602)
}
