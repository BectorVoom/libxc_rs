//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 936/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk936(t30713: f64, t4166: f64, t1484: f64, t22690: f64, t23122: f64, t6619: f64, t23083: f64, t32837: f64, t23062: f64, t32834: f64, t1509: f64, t23109: f64, t23110: f64, t232: f64, t59: f64) -> (f64, f64, f64, f64, f64) {
    let t118532 = t4166 * t30713;
    let t118573 = t23122 * t22690 * t6619 * t1484;
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    (t118532, t118573, t118578, t118580, t118586)
}
