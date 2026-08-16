//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 732/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk732(t1988: f64, t6883: f64, t131: f64, t209: f64, t547: f64, t1878: f64) -> (f64, f64, f64, f64) {
    let t6884 = t6883 * t1988;
    let t6885 = 0.19190897446562641759e-1_f64 * t6884;
    let t6887 = t547 * t131 * t209;
    let t6888 = t1878 * t6887;
    (t6884, t6885, t6887, t6888)
}
