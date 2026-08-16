//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1109/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1109(t1967: f64, t21455: f64, t7810: f64, t883: f64, t21460: f64, t20671: f64, t22538: f64, t22984: f64, t23183: f64, t7391: f64, t1457: f64, t7722: f64) -> (f64, f64, f64, f64, f64) {
    let t28936 = t7810 * t1967 * t883 * t21455;
    let t28940 = t7810 * t1967 * t883 * t21460;
    let t28944 = 0.17041300423964777634e0_f64 * t22538 * t20671 * t22984;
    let t28946 = 0.17875244975925213335e0_f64 * t23183 * t7391;
    let t28976 = t1457 * t7722;
    (t28936, t28940, t28944, t28946, t28976)
}
