//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 315/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk315(t1139: f64, t1204: f64, t1278: f64, t1282: f64, t1291: f64, t187: f64, t437: f64, t828: f64, t89: f64) -> (f64, f64) {
    let t1295 = t1139 - t1204 + t187 * (t1278 * t437 - t1282 * t1291 - t1139 + t1204);
    let t1646 = -t89 - t828;
    (t1295, t1646)
}
