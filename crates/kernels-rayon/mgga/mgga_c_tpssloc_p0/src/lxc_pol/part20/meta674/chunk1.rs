//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2543/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2543(t11269: f64, t3313: f64, t4785: f64, t11191: f64, t1670: f64, t44075: f64, t44077: f64, t11403: f64, t14838: f64, t11407: f64, t14850: f64, t44159: f64, t4745: f64) -> (f64, f64, f64, f64, f64) {
    let t51466 = 0.16081979498692535067e2_f64 * t3313 * t4785 * t11269;
    let t51470 = 0.24955700379505800916e5_f64 * t44075 * t1670 * t44077 * t11191;
    let t51472 = 6.0_f64 * t14838 * t11403;
    let t51474 = 0.48245938496077605201e2_f64 * t14850 * t11407;
    let t51476 = 6.0_f64 * t44159 * t4745;
    (t51466, t51470, t51472, t51474, t51476)
}
