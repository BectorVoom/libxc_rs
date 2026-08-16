//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3093/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3093(t12640: f64, t488: f64, t17588: f64, t3172: f64, t3711: f64, t1261: f64, t17699: f64, t17720: f64, t3647: f64, t12904: f64, t5274: f64, t12959: f64, t17505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56707 = t12640 * t488;
    let t56713 = t3711 * t3172 * t17588;
    let t56718 = t1261 * t3172 * t17699;
    let t56720 = t3647 * t17720;
    let t56726 = t5274 * t12904;
    let t56727 = 0.14291339372689912324e-3_f64 * t56726;
    let t56728 = t17505 * t12959;
    (t56707, t56713, t56718, t56720, t56727, t56728)
}
