//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2119/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2119(t29654: f64, t686: f64, t72: f64, t25387: f64, t25375: f64, t29610: f64, t27183: f64, t27199: f64, t92935: f64, t93112: f64, t93116: f64, t93138: f64, t93142: f64, t98918: f64, t98920: f64, t99127: f64, t99147: f64, t99163: f64, t99166: f64) -> (f64, f64) {
    let t106120 = t29654 * t72 * t686;
    let t106121 = t25387 * t106120;
    let t106123 = t25375 * t106120;
    let t106128 = t29610 * t72 * t686;
    let t106129 = t25387 * t106128;
    let t106134 = 0.65049603595885220126e-3_f64 * t92935 + t98918 + 0.13009920719177044025e-2_f64 * t98920 - t99127 + 0.25702851531048074406e-1_f64 * t106121 - 0.14456046980341999104e-1_f64 * t106123 - 0.24093411633903331839e-3_f64 * t93112 - 0.24093411633903331839e-3_f64 * t93116 + t99147 + 0.51405703062096148813e-1_f64 * t106129 + t99163 - 0.14634331517634470219e-1_f64 * t99166 + t93138 - t93142 + 0.17347256376410398924e1_f64 * t27199 * t27183;
    (t106128, t106134)
}
