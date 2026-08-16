//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 23/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk23(t6: f64, t97: f64, t95: f64, t64: f64, t80: f64, t87: f64, t90: f64, t72: f64, t75: f64) -> (f64, f64, f64, f64) {
    let t98 = t6 * t97;
    let t99 = t95 * t98;
    let t101 = -0.59778596625315888114e-2_f64 * t64 + 0.1317375e-2_f64 * t80 - 0.23775e-3_f64 * t87 + 0.64744236347453835951e-5_f64 * t90 - 0.540140625e-6_f64 * t99;
    let t103 = 0.11713266981940447749e-2_f64 * t64 * t72 - t75 * t101;
    (t98, t99, t101, t103)
}
