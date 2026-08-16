//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 567/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk567(t2136: f64, t7494: f64, t649: f64, t833: f64, t27: f64, t2134: f64, t504: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t7495 = t7494 * t2136;
    let t7496 = 0.20455996240684006296e-1_f64 * t7495;
    let t7497 = t649 * t833;
    let t7498 = t27 * t7497;
    let t7499 = t2134 * t7498;
    let t7500 = 0.10227998120342003148e-1_f64 * t7499;
    let t7501 = t504 * t880;
    (t7496, t7498, t7500, t7501)
}
