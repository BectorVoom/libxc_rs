//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 751/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk751(t1695: f64, t2517: f64, t1699: f64, t926: f64, t7088: f64, t7090: f64, t7093: f64, t7096: f64, t2524: f64, t471: f64, t64: f64, t90: f64, t931: f64) -> (f64, f64, f64, f64) {
    let t7098 = t2517 * t1695;
    let t7100 = t926 * t1699;
    let t7102 = 189.0_f64 / 512.0_f64 * t7088 - 483.0_f64 / 16384.0_f64 * t7090 + 147.0_f64 / 1048576.0_f64 * t7093 - 49.0_f64 / 1048576.0_f64 * t7096 + 161.0_f64 / 16384.0_f64 * t7098 - 63.0_f64 / 512.0_f64 * t7100;
    let t7112 = t7102 * t471 - 8.0_f64 / 3.0_f64 * t2524 * t64 + 4.0_f64 / 3.0_f64 * t931 * t90 + 63.0_f64 / 512.0_f64 * t7088 - 49.0_f64 / 16384.0_f64 * t7090 + 49.0_f64 / 49152.0_f64 * t7098 - 21.0_f64 / 512.0_f64 * t7100;
    (t7098, t7100, t7102, t7112)
}
