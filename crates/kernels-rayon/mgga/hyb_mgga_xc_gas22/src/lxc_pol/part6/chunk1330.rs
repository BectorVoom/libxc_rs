//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1330/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1330(t20694: f64, t20703: f64, t20706: f64, t20853: f64, t20867: f64, t28850: f64, t28853: f64, t28856: f64, t28859: f64, t28862: f64, t28866: f64, t28872: f64) -> f64 {
    let t28996 = 0.3071625e0_f64 * t28850 - 0.59793333333333333334e0_f64 * t28853 + 0.8969e0_f64 * t28856 + 0.39862222222222222223e0_f64 * t28859 + 0.27385555555555555555e0_f64 * t28862 + 0.49294e0_f64 * t28866 + t20867 + 0.27385555555555555556e0_f64 * t20694 + t20853 - 0.18602370370370370371e1_f64 * t20703 + 0.39862222222222222223e0_f64 * t20706 + 0.142419375e1_f64 * t28872;
    t28996
}
