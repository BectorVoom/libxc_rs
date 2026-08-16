//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1290/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1290(t10212: f64, t23894: f64, t3138: f64, t10158: f64, t10163: f64, t10205: f64, t2002: f64, t20218: f64, t2024: f64, t20252: f64, t20255: f64, t20258: f64, t2027: f64, t2028: f64, t23872: f64, t23905: f64, t23923: f64, t23925: f64, t27941: f64, t27955: f64, t27957: f64, t27962: f64, t27968: f64, t27976: f64, t3140: f64, t675: f64, t684: f64, t687: f64, t8511: f64, t8513: f64, t8526: f64) -> f64 {
    let t27979 = t3138 * t23894 * t10212;
    let t27990 = -t2024 * t2027 * t10158 * t2028 / 48.0_f64 - t684 * t687 * t27941 * t675 / 16.0_f64 - t684 * t687 * t10163 * t2002 / 32.0_f64 - t2024 * t2027 * t10163 * t2028 / 24.0_f64 - t27955 / 96.0_f64 - t8511 * t3140 * t27957 / 4.0_f64 + t27962 / 24.0_f64 + t20218 / 216.0_f64 + t20252 / 144.0_f64 - 5.0_f64 / 432.0_f64 * t20255 + t20258 / 288.0_f64 + t8526 * t3140 * t27968 / 16.0_f64 + 7.0_f64 / 18.0_f64 * t23872 * t8513 * t27957 - 7.0_f64 / 216.0_f64 * t27976 - 7.0_f64 / 36.0_f64 * t27979 - 7.0_f64 / 72.0_f64 * t8511 * t23905 * t10205 - 7.0_f64 / 144.0_f64 * t8511 * t8513 * t27968 - 35.0_f64 / 216.0_f64 * t23923 * t23925 * t27957;
    t27990
}
