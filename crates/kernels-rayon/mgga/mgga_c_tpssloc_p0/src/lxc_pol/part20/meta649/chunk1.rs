//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2389/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2389(t2807: f64, t896: f64, t13637: f64, t41680: f64, t41713: f64, t47777: f64, t48153: f64, t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64) -> (f64, f64, f64) {
    let t49039 = t2807 * t896;
    let t49040 = t13637 * t49039;
    let t49042 = 0.19931111111111111112e0_f64 * t41680 - 0.59793333333333333333e0_f64 * t41713 + 0.35876000000000000001e1_f64 * t47777 + 0.197176e1_f64 * t48153 + 0.5477111111111111111e0_f64 * t48155 - 0.91285185185185185185e-1_f64 * t48157 - 0.65725333333333333332e0_f64 * t48159 - 0.32862666666666666666e0_f64 * t48161 - 0.32862666666666666667e0_f64 * t48163 + 0.10954222222222222222e0_f64 * t48165 + 0.54771111111111111111e-1_f64 * t48167 - 0.230371875e0_f64 * t49040;
    (t49039, t49040, t49042)
}
