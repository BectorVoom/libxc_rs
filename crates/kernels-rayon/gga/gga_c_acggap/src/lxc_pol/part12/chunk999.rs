//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 999/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk999(t1165: f64, t5249: f64, t604: f64, t7493: f64, t7433: f64, t8869: f64, t7839: f64, t2068: f64, t5080: f64, t1411: f64, t1992: f64, t7585: f64, t7842: f64) -> (f64, f64, f64, f64, f64) {
    let t33839 = t7493 * t1165 * t604 * t5249;
    let t33841 = t7433 * t8869;
    let t33843 = t7839 * t8869;
    let t33847 = t2068 * t1165 * t604 * t5080;
    let t33851 = t7585 * t7842 * t1992 * t1411;
    (t33839, t33841, t33843, t33847, t33851)
}
