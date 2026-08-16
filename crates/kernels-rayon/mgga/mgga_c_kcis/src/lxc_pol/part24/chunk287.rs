//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 287/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk287(t1187: f64, t1189: f64, t279: f64, t383: f64, t280: f64, t251: f64) -> (f64, f64, f64) {
    let t1190 = t1187 * t1189;
    let t1192 = t383 * t279;
    let t1194 = 1.0_f64 / t280 / t1192;
    let t1195 = t1194 * t251;
    (t1190, t1194, t1195)
}
