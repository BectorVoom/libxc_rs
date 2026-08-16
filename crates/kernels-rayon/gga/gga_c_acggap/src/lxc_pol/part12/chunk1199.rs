//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1199/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1199(t35549: f64, t35552: f64, t35556: f64, t35560: f64, t31363: f64, t31374: f64, t31382: f64, t31386: f64, t32795: f64, t32796: f64, t32799: f64, t32800: f64, t32803: f64, t35545: f64, t35564: f64, t35567: f64, t35569: f64, t35573: f64) -> f64 {
    let t37605 = 0.12579236915841660828e-2_f64 * t35549;
    let t37606 = 0.18868855373762491241e-2_f64 * t35552;
    let t37607 = 0.12579236915841660828e-2_f64 * t35556;
    let t37610 = 35.0_f64 / 216.0_f64 * t35560;
    let t37617 = 0.34299214494455789578e-2_f64 * t35545 - t37605 + t37606 - t37607 - 0.31448092289604152068e-2_f64 * t31363 + 0.3361875e0_f64 * t31374 + t37610 - t32795 - t32796 + 13.0_f64 / 24.0_f64 * t31382 + 0.17149607247227894789e-2_f64 * t31386 + t32799 - t32800 - t32803 + 0.27439371595564631662e-1_f64 * t35564 + 0.42874018118069736972e-3_f64 * t35567 + 0.62896184579208304138e-2_f64 * t35569 - 0.62896184579208304138e-2_f64 * t35573;
    t37617
}
