//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 204/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk204(t31: f64, t607: f64, t65: f64, t34: f64, t36: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t608 = t31 * t607;
    let t609 = t608 * t65;
    let t612 = t34 * rho0;
    let t614 = 1.0_f64 / t36 / t612;
    let t615 = sigma0 * t614;
    (t608, t609, t612, t614, t615)
}
