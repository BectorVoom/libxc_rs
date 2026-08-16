//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1280/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1280(t10213: f64, t241: f64, t136: f64, t41667: f64, t41671: f64, t908: f64, t10319: f64, t699: f64, t10313: f64, t2826: f64, t41649: f64, t41654: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41880 = t241 * t10213;
    let t41882 = t136 * t41880 * t41667;
    let t41885 = t136 * t908 * t41671;
    let t41887 = t699 * t10319;
    let t41889 = t699 * t10313;
    let t41892 = t136 * t2826 * t41649;
    let t41904 = 280.0_f64 / 81.0_f64 * t41654;
    (t41882, t41885, t41887, t41889, t41892, t41904)
}
