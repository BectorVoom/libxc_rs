//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1273/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1273(t3886: f64, t6992: f64, t22716: f64, t8459: f64, t22779: f64, t31162: f64, t22817: f64, t794: f64, t8462: f64, t1369: f64, t31176: f64, t22804: f64, t31156: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113946 = t3886 * t6992;
    let t113963 = 0.12793931631041761173e0_f64 * t22716 * t8459;
    let t113966 = t22779 * t31162;
    let t113967 = 0.11304371706359309439e-1_f64 * t113966;
    let t113981 = t22817 * t794 * t8462;
    let t113987 = t31176 * t1369;
    let t113988 = 7.0_f64 / 288.0_f64 * t113987;
    let t114000 = t22804 * t31156;
    (t113946, t113963, t113967, t113981, t113988, t114000)
}
