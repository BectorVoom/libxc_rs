//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1999/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1999(t22724: f64, t22927: f64, t22642: f64, t22643: f64, t6907: f64, t22644: f64, t81152: f64, t6891: f64, t81195: f64, t1372: f64, t212: f64, t6890: f64) -> (f64, f64, f64, f64, f64) {
    let t81264 = t22724 * t22927;
    let t81267 = t22642 * t22643 * t6907;
    let t81281 = t81152 * t22644;
    let t81282 = 0.98696044010893586188e-1_f64 * t81281;
    let t81284 = t81195 * t22643 * t6891;
    let t81311 = t22642 * t212 * t1372 * t6890;
    (t81264, t81267, t81282, t81284, t81311)
}
