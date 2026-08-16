//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 250/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk250(t659: f64, t95: f64, t103: f64, t100: f64, t657: f64, t92: f64, t96: f64) -> (f64, f64, f64) {
    let t660 = t95 * t659;
    let t662 = -t659;
    let t663 = t103 * t662;
    let t666 = 5.0_f64 / 3.0_f64 * t100 * t663 - 5.0_f64 / 3.0_f64 * t657 * t96 + 5.0_f64 / 3.0_f64 * t92 * t660;
    (t662, t663, t666)
}
