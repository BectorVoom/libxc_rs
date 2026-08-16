//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2434/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2434(t17513: f64, t49489: f64, t10661: f64, t21253: f64, t912: f64, t2842: f64, t4395: f64, t5695: f64, t10702: f64, t21268: f64, t10817: f64, t21315: f64) -> (f64, f64, f64, f64, f64) {
    let t69288 = 0.2894756309764656312e3_f64 * t49489 * t17513;
    let t69291 = 24.0_f64 * t10661 * t21253 * t912;
    let t69294 = 18.0_f64 * t2842 * t5695 * t4395;
    let t69297 = 0.57895126195293126241e3_f64 * t10702 * t21268 * t912;
    let t69299 = 6.0_f64 * t10817 * t21315;
    (t69288, t69291, t69294, t69297, t69299)
}
