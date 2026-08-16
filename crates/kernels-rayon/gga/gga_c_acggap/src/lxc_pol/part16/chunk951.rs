//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 951/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk951(t1165: f64, t5249: f64, t604: f64, t7493: f64, t7433: f64, t8869: f64, t7839: f64, t1411: f64, t1992: f64, t7585: f64, t7842: f64, t31699: f64, t8665: f64) -> (f64, f64, f64, f64, f64) {
    let t33839 = t7493 * t1165 * t604 * t5249;
    let t33840 = 0.15724046144802076034e-2_f64 * t33839;
    let t33841 = t7433 * t8869;
    let t33842 = 0.18868855373762491241e-2_f64 * t33841;
    let t33843 = t7839 * t8869;
    let t33844 = 0.31448092289604152068e-3_f64 * t33843;
    let t33851 = t7585 * t7842 * t1992 * t1411;
    let t33852 = 0.20965394859736101378e-3_f64 * t33851;
    let t33853 = t31699 * t8665;
    (t33840, t33842, t33844, t33852, t33853)
}
