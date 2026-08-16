//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2386/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2386(t48960: f64, t48980: f64, t901: f64, t41662: f64, t41675: f64, t41678: f64, t41682: f64, t41684: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64) -> (f64, f64, f64) {
    let t48981 = t48960 + t48980;
    let t48982 = t901 * t48981;
    let t48990 = 0.99655555555555555557e-1_f64 * t41662 + 0.79724444444444444447e0_f64 * t41675 - 0.39862222222222222222e0_f64 * t41678 + 0.59793333333333333333e0_f64 * t41682 + 0.93011851851851851855e0_f64 * t41684 + 0.3071625e0_f64 * t48982 + 0.73028148148148148149e0_f64 * t41863 - 0.10954222222222222222e0_f64 * t41865 - 0.27385555555555555556e0_f64 * t41870 - 0.91285185185185185185e-1_f64 * t41872 + 0.54771111111111111111e-1_f64 * t41874 + 0.24342716049382716049e-1_f64 * t41876;
    (t48981, t48982, t48990)
}
