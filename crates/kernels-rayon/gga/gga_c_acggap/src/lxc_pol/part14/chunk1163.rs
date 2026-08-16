//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1163/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1163(t8463: f64, t8480: f64, t8652: f64, t31341: f64, t31377: f64, t31381: f64, t31382: f64, t31386: f64, t31390: f64, t31392: f64, t31407: f64, t35550: f64, t35553: f64, t35557: f64, t37610: f64, t40043: f64, t40045: f64, t40047: f64, t40050: f64, t40054: f64) -> f64 {
    let t40057 = t8463 * t8480 * t8652;
    let t40059 = t31341 - t35550 + t35553 - 0.62896184579208304136e-3_f64 * t40043 - t35557 + t37610 - t31377 - t31381 - 0.420234375e-1_f64 * t40045 - 0.28015625e-1_f64 * t40047 + t40050 / 8.0_f64 + 13.0_f64 / 96.0_f64 * t31382 + 0.42874018118069736972e-3_f64 * t31386 + t31390 - t31392 - t31407 - 0.34299214494455789578e-1_f64 * t40054 + 0.64311027177104605458e-2_f64 * t40057;
    t40059
}
