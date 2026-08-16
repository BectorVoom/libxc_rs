//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 993/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk993(t9112: f64, t9115: f64, t6969: f64, t6972: f64, t9119: f64, t9123: f64, t9127: f64, t9136: f64, t9138: f64, t9140: f64, t9143: f64, t9145: f64) -> (f64, f64, f64) {
    let t9217 = 0.41678e0_f64 * t9112;
    let t9218 = 0.41678e0_f64 * t9115;
    let t9229 = -t9217 - t9218 + 0.312585e0_f64 * t9119 + 0.62517e0_f64 * t9123 + 0.312585e0_f64 * t9127 + 0.13772666666666666667e1_f64 * t6969 - 0.516475e0_f64 * t6972 + 0.3529725e1_f64 * t9136 + 0.6311625e0_f64 * t9138 - 0.17648625e1_f64 * t9140 + 0.6311625e0_f64 * t9143 + 0.31558125e0_f64 * t9145;
    (t9217, t9218, t9229)
}
