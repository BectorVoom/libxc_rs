//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2411/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2411(t41662: f64, t41675: f64, t41678: f64, t41682: f64, t41684: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t48982: f64) -> f64 {
    let t49345 = 0.17215833333333333333e0_f64 * t41662 + 0.13772666666666666666e1_f64 * t41675 - 0.68863333333333333332e0_f64 * t41678 + 0.103295e1_f64 * t41682 + 0.16068111111111111111e1_f64 * t41684 + 0.6311625e0_f64 * t48982 + 0.92617777777777777776e0_f64 * t41863 - 0.13892666666666666667e0_f64 * t41865 - 0.34731666666666666666e0_f64 * t41870 - 0.11577222222222222222e0_f64 * t41872 + 0.69463333333333333333e-1_f64 * t41874 + 0.30872592592592592592e-1_f64 * t41876;
    t49345
}
