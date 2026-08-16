//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2400/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2400(t41662: f64, t41675: f64, t41678: f64, t41682: f64, t41684: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t48982: f64) -> f64 {
    let t49167 = 0.10064166666666666667e0_f64 * t41662 + 0.80513333333333333335e0_f64 * t41675 - 0.40256666666666666668e0_f64 * t41678 + 0.60385000000000000002e0_f64 * t41682 + 0.93932222222222222223e0_f64 * t41684 + 0.16504875e0_f64 * t48982 + 0.73586666666666666668e0_f64 * t41863 - 0.11038e0_f64 * t41865 - 0.27595e0_f64 * t41870 - 0.91983333333333333335e-1_f64 * t41872 + 0.5519e-1_f64 * t41874 + 0.24528888888888888889e-1_f64 * t41876;
    t49167
}
