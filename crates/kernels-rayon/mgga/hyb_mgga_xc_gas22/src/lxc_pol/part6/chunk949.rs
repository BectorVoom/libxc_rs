//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 949/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk949(t8651: f64, t2194: f64, t3329: f64, t791: f64, t2200: f64, t3324: f64, t2206: f64, t3335: f64, t6530: f64, t6533: f64, t6616: f64, t6619: f64, t6622: f64, t6655: f64, t8648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8652 = 0.60385e0_f64 * t8651;
    let t8653 = t2194 * t3329;
    let t8654 = t8653 * t791;
    let t8656 = t3324 * t2200;
    let t8658 = t2206 * t3329;
    let t8659 = t8658 * t791;
    let t8661 = t3335 * t2200;
    let t8668 = 0.905775e0_f64 * t8648 - t8652 - 0.258925e1_f64 * t8654 - 0.1294625e1_f64 * t8656 + 0.16504875e0_f64 * t8659 + 0.82524375e-1_f64 * t8661 + 0.80513333333333333334e0_f64 * t6530 - 0.301925e0_f64 * t6533 - t6655 + 0.5519e0_f64 * t6616 - 0.16557e0_f64 * t6619 - 0.16557e0_f64 * t6622;
    (t8652, t8654, t8656, t8659, t8661, t8668)
}
