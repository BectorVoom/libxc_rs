//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1352/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1352(t136: f64, t2826: f64, t76597: f64, t76593: f64, t41880: f64, t76572: f64, t76576: f64, t908: f64, t76589: f64, t10304: f64, t76581: f64, t76585: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76877 = t136 * t2826 * t76597;
    let t76880 = t136 * t2826 * t76593;
    let t76887 = t136 * t41880 * t76572;
    let t76890 = t136 * t908 * t76576;
    let t76893 = t136 * t2826 * t76589;
    let t76896 = t136 * t10304 * t76581;
    let t76899 = t136 * t10304 * t76585;
    (t76877, t76880, t76887, t76890, t76893, t76896, t76899)
}
