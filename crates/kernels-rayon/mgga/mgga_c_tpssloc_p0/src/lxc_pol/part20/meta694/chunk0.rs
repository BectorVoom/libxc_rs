//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2643/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2643(t28: f64, t3673: f64, t584: f64, t1081: f64, t3231: f64, t16: f64, t5181: f64, t591: f64, t11122: f64, t12000: f64, t12001: f64, t1302: f64, t16003: f64, t16006: f64, t1649: f64, t2: f64, t3711: f64, t39877: f64, t5178: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t53832 = t584 * t3673;
    let t53835 = t1081 * t3231;
    let t53841 = t16 * t1081;
    let t53844 = t584 * t3231;
    let t53852 = 16.0_f64 * t5181 * t591;
    let t53854 = piecewise3(t29, 0.0_f64, -56.0_f64 / 81.0_f64 * t39877 * t1649 * t12001 - 16.0_f64 / 9.0_f64 * t12000 * t2 * t53832 + 8.0_f64 / 9.0_f64 * t16003 * t53835 + 4.0_f64 / 3.0_f64 * t3711 * t584 * t1081 - 4.0_f64 * t16006 * t53841 + 4.0_f64 / 3.0_f64 * t16006 * t53844 - 2.0_f64 / 9.0_f64 * t5178 * t11122 + 8.0_f64 * t1302 * t16 - t53852);
    (t53832, t53835, t53841, t53844, t53854)
}
