//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 573/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk573(t1131: f64, t1134: f64, t1655: f64, t1662: f64, t1665: f64, t1668: f64) -> f64 {
    let t1682 = 0.3529725e1_f64 * t1662 - t1131 + 0.516475e0_f64 * t1655 + 0.6311625e0_f64 * t1665 - t1134 + 0.104195e0_f64 * t1668;
    t1682
}
