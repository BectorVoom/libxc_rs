//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1169/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1169(t20994: f64, t6581: f64, t1894: f64, t20800: f64, t236: f64, t6591: f64, t20974: f64, t23146: f64, t1509: f64, t232: f64, t25119: f64, t5527: f64, t815: f64) -> (f64, f64, f64, f64) {
    let t105372 = t6581 * t20994;
    let t105376 = t6591 * t1894 * t236 * t20800;
    let t105381 = t23146 * t20974;
    let t105387 = t25119 * t815 * t5527 * t1509 * t232;
    (t105372, t105376, t105381, t105387)
}
