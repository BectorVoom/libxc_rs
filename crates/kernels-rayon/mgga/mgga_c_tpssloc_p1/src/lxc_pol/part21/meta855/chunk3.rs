//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3093/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3093(t1099: f64, t1118: f64, t63847: f64, t63881: f64, t63916: f64, t64011: f64, t64027: f64, t64049: f64, t64066: f64, t64094: f64, t3356: f64, t6031: f64) -> (f64, f64) {
    let t64100 = 1.0_f64 * t1099 * (t63847 + t63881 + t63916 + t64011 + t64027 + t64049 + t64066 + t64094) * t1118;
    let t64103 = t6031 * t3356;
    (t64100, t64103)
}
