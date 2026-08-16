//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2228/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2228(t16257: f64, t26309: f64, t5293: f64, t80820: f64, t5259: f64, t80816: f64, t16244: f64, t22833: f64, t5303: f64, t16366: f64, t16370: f64, t91094: f64, t91096: f64, t91098: f64, t91101: f64, t91103: f64, t91105: f64, t91107: f64, t91109: f64, t91114: f64, t91116: f64) -> f64 {
    let t91118 = t26309 * t16257;
    let t91120 = t80820 * t5293;
    let t91121 = 7.0_f64 / 1152.0_f64 * t91120;
    let t91122 = t80816 * t5259;
    let t91124 = t22833 * t16244;
    let t91126 = t80816 * t5303;
    let t91128 = t22833 * t16366;
    let t91130 = t22833 * t16370;
    let t91132 = t91094 / 384.0_f64 + t91096 / 384.0_f64 + t91098 / 768.0_f64 + t91101 / 192.0_f64 - 5.0_f64 / 384.0_f64 * t91103 + t91105 / 256.0_f64 - t91107 / 1536.0_f64 - t91109 / 768.0_f64 - t91114 + t91116 / 384.0_f64 + t91118 / 384.0_f64 + t91121 + t91122 / 192.0_f64 + t91124 / 192.0_f64 + t91126 / 192.0_f64 + t91128 / 192.0_f64 + t91130 / 384.0_f64;
    t91132
}
