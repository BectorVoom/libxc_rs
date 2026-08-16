//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2310/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2310(t16606: f64, t17120: f64, t1877: f64, t40764: f64, t40766: f64, t4255: f64, t4303: f64, t4314: f64, t46292: f64, t67176: f64, t67178: f64, t67180: f64, t67183: f64, t67186: f64, t67191: f64) -> f64 {
    let t67195 = 18.0_f64 * t16606 * t4255 * t4314 + 6.0_f64 * t17120 * t1877 * t4303 + t40764 + t40766 + t46292 - t67176 + t67178 + t67180 + t67183 + t67186 + t67191;
    t67195
}
