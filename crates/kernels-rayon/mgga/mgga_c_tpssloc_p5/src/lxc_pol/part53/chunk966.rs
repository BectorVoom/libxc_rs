//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 966/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk966(t23083: f64, t32837: f64, t23062: f64, t32834: f64, t1509: f64, t23109: f64, t23110: f64, t232: f64, t59: f64, t1516: f64, t30720: f64, t30709: f64) -> (f64, f64, f64, f64, f64) {
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    let t118588 = t30720 * t1516;
    let t118590 = t30709 * t1516;
    (t118578, t118580, t118586, t118588, t118590)
}
