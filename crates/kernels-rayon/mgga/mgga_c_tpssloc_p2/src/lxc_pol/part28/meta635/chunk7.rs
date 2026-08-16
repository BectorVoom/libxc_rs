//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2018/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2018(t91000: f64, t91010: f64, t91113: f64, t91120: f64, t91094: f64, t91096: f64, t91098: f64, t91101: f64, t91103: f64, t91105: f64, t91107: f64, t91109: f64, t91116: f64, t91118: f64, t91122: f64, t91124: f64, t91126: f64, t91128: f64, t91130: f64) -> (f64, f64, f64) {
    let t93615 = 0.12793931631041761173e0_f64 * t91000;
    let t93618 = 0.15352717957250113407e0_f64 * t91010;
    let t93633 = 7.0_f64 / 288.0_f64 * t91113;
    let t93636 = 7.0_f64 / 576.0_f64 * t91120;
    let t93642 = t91094 / 192.0_f64 + t91096 / 192.0_f64 + t91098 / 384.0_f64 + t91101 / 96.0_f64 - 5.0_f64 / 192.0_f64 * t91103 + t91105 / 128.0_f64 - t91107 / 768.0_f64 - t91109 / 384.0_f64 - t93633 + t91116 / 192.0_f64 + t91118 / 192.0_f64 + t93636 + t91122 / 96.0_f64 + t91124 / 96.0_f64 + t91126 / 96.0_f64 + t91128 / 96.0_f64 + t91130 / 192.0_f64;
    (t93615, t93618, t93642)
}
