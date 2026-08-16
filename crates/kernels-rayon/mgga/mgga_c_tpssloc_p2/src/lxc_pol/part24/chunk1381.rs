//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1381/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1381(t23436: f64, t3113: f64, t1036: f64, t23465: f64, t3082: f64, t6759: f64, t344: f64, t607: f64, t1009: f64, t6740: f64, t23473: f64, t23509: f64, t25651: f64) -> (f64, f64, f64, f64, f64) {
    let t82880 = t3113 * t23436;
    let t82883 = t23465 * t1036;
    let t82885 = t6759 * t3082;
    let t82890 = t607 * t344;
    let t82892 = t6740 * t82890 * t1009;
    let t82893 = t82892 * t23473;
    let t82895 = t23509 * t25651;
    (t82880, t82883, t82885, t82893, t82895)
}
