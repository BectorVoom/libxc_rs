//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 629/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk629(t228: f64, t68: f64, t2627: f64, t226: f64, t814: f64, t193: f64, t200: f64) -> (f64, f64, f64, f64) {
    let t4225 = t228 * t68;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4290 = t68 * t814;
    let t4291 = t226 * t4290;
    let t4314 = t193 * t200;
    (t4225, t4281, t4291, t4314)
}
