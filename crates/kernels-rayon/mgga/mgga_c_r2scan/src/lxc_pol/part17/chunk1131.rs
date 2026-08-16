//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1131/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1131(t12025: f64, t12027: f64, t12030: f64, t12034: f64, t12037: f64, t12039: f64, t12040: f64, t12046: f64, t12048: f64, t12049: f64, t12053: f64, t12055: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41116 = 45.0_f64 / 32.0_f64 * t12025;
    let t41117 = 5.0_f64 / 8.0_f64 * t12027;
    let t41118 = 5.0_f64 / 8.0_f64 * t12030;
    let t41119 = t12034 / 2.0_f64;
    let t41120 = 5.0_f64 / 8.0_f64 * t12037;
    let t41121 = 2.0_f64 * t12039;
    let t41122 = t12040 / 2.0_f64;
    let t41123 = 3.0_f64 / 2.0_f64 * t12046;
    let t41124 = 2.0_f64 * t12048;
    let t41126 = t12049 / 2.0_f64;
    let t41127 = t12053 / 2.0_f64;
    let t41128 = 2.0_f64 * t12055;
    (t41116, t41117, t41118, t41119, t41120, t41121, t41122, t41123, t41124, t41126, t41127, t41128)
}
