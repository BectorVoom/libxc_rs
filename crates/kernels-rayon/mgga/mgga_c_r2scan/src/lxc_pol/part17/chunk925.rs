//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 925/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk925(t10710: f64, t6481: f64, t10708: f64, t10707: f64, t546: f64) -> (f64, f64, f64) {
    let t10711 = t10710 * t6481;
    let t10712 = t10708 * t10711;
    let t10713 = 0.14282990759302185292e-1_f64 * t10712;
    let t10728 = t546 * t10707;
    (t10711, t10713, t10728)
}
