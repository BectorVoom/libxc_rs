//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 565/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk565(t2136: f64, t7494: f64, t649: f64, t833: f64, t27: f64, t2134: f64, t504: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t7495 = t7494 * t2136;
    let t7497 = t649 * t833;
    let t7498 = t27 * t7497;
    let t7499 = t2134 * t7498;
    let t7501 = t504 * t880;
    (t7495, t7498, t7499, t7501)
}
