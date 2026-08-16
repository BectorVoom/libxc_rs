//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 945/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk945(t12: f64, t7335: f64, t5528: f64, t972: f64, t1837: f64, t8: f64, t1429: f64, t652: f64, t1643: f64, t1646: f64, t2732: f64, t2735: f64, t6771: f64, t82: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t7336 = 0.103295e1_f64 * t7335;
    let t7337 = t5528 * t972;
    let t7340 = t1837 * t8;
    let t7345 = t652 * t1429;
    let t7350 = piecewise3(t84, 0.0_f64, -28.0_f64 / 27.0_f64 * t7337 * t1643 + 16.0_f64 / 9.0_f64 * t7340 * t6771 + 4.0_f64 / 9.0_f64 * t2732 * t1646 - 2.0_f64 / 3.0_f64 * t7345 + 2.0_f64 * t2735 * t82);
    (t7336, t7337, t7340, t7345, t7350)
}
