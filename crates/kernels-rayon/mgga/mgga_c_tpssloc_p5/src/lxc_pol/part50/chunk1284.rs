//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1284/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1284(t119827: f64, t119863: f64, t120015: f64, t120059: f64, t120098: f64, t120667: f64, t120713: f64, t120755: f64, t1858: f64, t8496: f64, t2029: f64, t7758: f64) -> (f64, f64, f64) {
    let t120758 = t119827 + t119863 + t120015 + t120059 + t120098 + t120667 + t120713 + t120755;
    let t120762 = t8496 * t1858;
    let t120767 = t7758 * t2029;
    (t120758, t120762, t120767)
}
