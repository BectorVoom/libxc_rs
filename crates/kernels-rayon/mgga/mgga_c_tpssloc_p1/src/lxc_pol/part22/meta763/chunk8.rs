//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2578/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2578(t18915: f64, t4879: f64, t21938: f64, t3400: f64, t1164: f64, t4883: f64, t300: f64, t71310: f64, t1155: f64, t1695: f64, t51810: f64, t6084: f64) -> (f64, f64, f64, f64) {
    let t72061 = 0.17544670867903938621e1_f64 * t18915 * t4879;
    let t72062 = t3400 * t21938;
    let t72065 = 0.17315859105681463759e2_f64 * t1164 * t72062 * t4883;
    let t72067 = 0.19751673498613801407e-1_f64 * t300 * t71310;
    let t72071 = 0.10526802520742363173e2_f64 * t51810 * t1695 * t6084 * t1155;
    (t72061, t72065, t72067, t72071)
}
