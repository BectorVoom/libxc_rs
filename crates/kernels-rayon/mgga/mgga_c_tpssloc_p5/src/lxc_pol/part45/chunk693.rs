//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 693/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk693(t6505: f64, t6509: f64, t2235: f64, t608: f64, t33: f64, t6504: f64, t2240: f64, t641: f64, t645: f64, t72: f64, t2307: f64, t79: f64) -> (f64, f64, f64, f64, f64) {
    let t22516 = t6505 * t6509;
    let t22519 = t2235 * t608;
    let t22522 = t33 * t6504;
    let t22523 = t2240 * t22522;
    let t22527 = t72 * t641 * t645;
    let t22530 = t79 * t2307;
    (t22516, t22519, t22523, t22527, t22530)
}
