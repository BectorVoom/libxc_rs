//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1173/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1173(t1509: f64, t23109: f64, t23110: f64, t232: f64, t59: f64, t1516: f64, t30720: f64, t30709: f64, t4261: f64, t8343: f64, t32840: f64, t849: f64) -> (f64, f64, f64, f64, f64) {
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    let t118588 = t30720 * t1516;
    let t118590 = t30709 * t1516;
    let t118592 = t8343 * t4261;
    let t118594 = t32840 * t849;
    (t118586, t118588, t118590, t118592, t118594)
}
