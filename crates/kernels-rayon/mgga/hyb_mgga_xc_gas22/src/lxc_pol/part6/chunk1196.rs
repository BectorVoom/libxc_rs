//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1196/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1196(t22150: f64, t2638: f64, t1110: f64, t21846: f64, t2676: f64, t7245: f64, t2714: f64, t2723: f64, t2729: f64, t1046: f64, t2731: f64, t7435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22151 = 1.0_f64 / t22150;
    let t22153 = t2638 * t2638;
    let t22154 = 1.0_f64 / t22153;
    let t22157 = 0.91082604192152556044e5_f64 * t1110 * t22151 * t21846 * t22154;
    let t22158 = t7245 * t2676;
    let t22162 = 36.0_f64 * t2729 * t2714 * t2723;
    let t22166 = 0.64327917994770140268e2_f64 * t2729 * t7435 * t2731 * t1046;
    (t22151, t22154, t22157, t22158, t22162, t22166)
}
