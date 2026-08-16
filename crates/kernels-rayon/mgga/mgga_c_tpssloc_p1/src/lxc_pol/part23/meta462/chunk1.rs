//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1353/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353(t68500: f64, t68502: f64, t68504: f64, t68506: f64, t76877: f64, t76880: f64, t76887: f64, t76890: f64, t76893: f64, t76896: f64, t76899: f64, t136: f64, t76624: f64, t908: f64) -> (f64, f64) {
    let t76901 = t76877 / 6.0_f64 - 2.0_f64 * t76880 - 16.0_f64 / 81.0_f64 * t68500 - 4.0_f64 / 9.0_f64 * t68502 - 8.0_f64 / 3.0_f64 * t68504 + 8.0_f64 / 9.0_f64 * t68506 + 14.0_f64 / 81.0_f64 * t76887 + t76890 / 6.0_f64 + 2.0_f64 * t76893 - 8.0_f64 / 9.0_f64 * t76896 + 4.0_f64 / 9.0_f64 * t76899;
    let t76903 = t136 * t908 * t76624;
    (t76901, t76903)
}
