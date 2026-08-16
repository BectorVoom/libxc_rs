//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 924/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk924(t1509: f64, t23109: f64, t23110: f64, t232: f64, t59: f64, t1516: f64, t30720: f64, t112802: f64, t32844: f64, t1499: f64, t30719: f64, t8344: f64) -> (f64, f64, f64, f64) {
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    let t118588 = t30720 * t1516;
    let t118596 = t112802 * t32844;
    let t118602 = t1499 * t30719 * t8344;
    (t118586, t118588, t118596, t118602)
}
