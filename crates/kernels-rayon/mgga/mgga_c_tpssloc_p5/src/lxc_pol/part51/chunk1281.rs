//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1281/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1281(t31120: f64, t6883: f64, t31108: f64, t6897: f64, t794: f64, t114172: f64, t22892: f64, t6891: f64, t22573: f64, t8449: f64, t31338: f64, t81651: f64, t82074: f64) -> (f64, f64, f64, f64, f64) {
    let t114296 = t6883 * t31120;
    let t114299 = t6897 * t794 * t31108;
    let t114316 = t22892 * t114172 * t6891;
    let t114360 = t8449 * t22573;
    let t114592 = t81651 * t82074 * t31338;
    (t114296, t114299, t114316, t114360, t114592)
}
