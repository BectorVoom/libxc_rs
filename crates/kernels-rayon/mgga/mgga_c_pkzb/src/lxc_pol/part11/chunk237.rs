//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 237/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk237(t179: f64, t655: f64, t780: f64, t276: f64, t279: f64, t299: f64, t303: f64, t735: f64, t741: f64, t744: f64, t757: f64, t763: f64, t771: f64, t777: f64) -> f64 {
    let t782 = t179 * t780 * t655;
    let t785 = -t735 * t279 / 36.0_f64 + t741 - t276 * t744 / 96.0_f64 + 0.21437009059034868486e-3_f64 * t757 * t763 - 0.11433071498151929859e-2_f64 * t771 * t303 + t777 - 0.42874018118069736972e-3_f64 * t299 * t782;
    t785
}
