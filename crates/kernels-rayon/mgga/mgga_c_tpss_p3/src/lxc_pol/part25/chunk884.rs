//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 884/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk884(t2332: f64, t581: f64, t681: f64, t2204: f64, t2319: f64, t721: f64, t735: f64, t2214: f64, t719: f64, t2210: f64, t2222: f64, t2341: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8100 = t2332 * t581;
    let t8101 = t681 * t8100;
    let t8110 = t2319 * t2204 * t721;
    let t8112 = 0.35089341735807877242e1_f64 * t735 * t8110;
    let t8114 = t2214 * t719;
    let t8115 = t2210 * t2204 * t8114;
    let t8117 = 0.51947577317044391277e2_f64 * t735 * t8115;
    let t8118 = t2341 * t2222;
    (t8101, t8110, t8112, t8115, t8117, t8118)
}
