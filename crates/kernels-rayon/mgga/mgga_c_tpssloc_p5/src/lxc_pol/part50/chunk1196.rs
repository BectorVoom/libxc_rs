//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1196/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1196(t118509: f64, t118634: f64, t118668: f64, t118793: f64, t118832: f64, t118878: f64, t118914: f64, t118945: f64, t870: f64, t7540: f64, t868: f64, t25373: f64) -> (f64, f64, f64, f64) {
    let t118948 = t118509 + t118634 + t118668 + t118793 + t118832 + t118878 + t118914 + t118945;
    let t118949 = t118948 * t870;
    let t118953 = t7540 * t868;
    let t118954 = t25373 * t118953;
    (t118948, t118949, t118953, t118954)
}
