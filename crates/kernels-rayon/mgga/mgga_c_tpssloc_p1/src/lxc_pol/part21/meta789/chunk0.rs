//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2747/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2747(t17109: f64, t870: f64, t46206: f64, t12939: f64, t16716: f64, t2250: f64, t16558: f64, t184: f64, t4194: f64, t607: f64, t16619: f64, t16689: f64, t2430: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57932 = t17109 * t870;
    let t57936 = 16.0_f64 * t46206;
    let t57939 = 24.0_f64 * t12939 * t16716 * t2250;
    let t57943 = 24.0_f64 * t4194 * t184 * t16558 * t607;
    let t57946 = 12.0_f64 * t4194 * t16619 * t2250;
    let t57947 = t16689 * t2430;
    (t57932, t57936, t57939, t57943, t57946, t57947)
}
