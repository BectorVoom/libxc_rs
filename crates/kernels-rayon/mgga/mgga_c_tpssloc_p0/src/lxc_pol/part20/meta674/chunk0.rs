//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2542/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2542(t11269: f64, t1671: f64, t3264: f64, t11191: f64, t15067: f64, t43969: f64, t15060: f64, t3307: f64, t3313: f64, t11277: f64, t4781: f64, t11275: f64, t3265: f64) -> (f64, f64, f64, f64) {
    let t51453 = 2.0_f64 * t3264 * t1671 * t11269;
    let t51456 = 0.62071215503128080361e4_f64 * t43969 * t15067 * t11191;
    let t51459 = 0.48245938496077605201e2_f64 * t3313 * t15060 * t3307;
    let t51460 = t4781 * t11277;
    let t51463 = 0.1551780387578202009e4_f64 * t11275 * t51460 * t3265;
    (t51453, t51456, t51459, t51463)
}
