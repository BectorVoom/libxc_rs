//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1217/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1217(t1433: f64, t641: f64, t8513: f64, t4017: f64, t79: f64, t4021: f64, t8307: f64, t32781: f64, t532: f64, t1983: f64, t6879: f64, t33160: f64, t6876: f64) -> (f64, f64, f64, f64, f64) {
    let t119971 = t8513 * t641 * t1433;
    let t119975 = t8513 * t79 * t4017;
    let t119990 = t8513 * t8307 * t4021;
    let t119999 = t532 * t32781;
    let t120002 = 3.0_f64 * t1983 * t119999 * t6879;
    let t120008 = 3.0_f64 * t6876 * t33160;
    (t119971, t119975, t119990, t120002, t120008)
}
