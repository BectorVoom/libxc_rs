//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 343/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk343(t174: f64, t1139: f64, t1204: f64, t1278: f64, t1282: f64, t1291: f64, t187: f64, t437: f64, t236: f64, t833: f64, t447: f64, t637: f64, t237: f64, t318: f64, t451: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t1295 = t1139 - t1204 + t187 * (t1278 * t437 - t1282 * t1291 - t1139 + t1204);
    let t1296 = t236 * t1295;
    let t1299 = piecewise3(t175, 0.0_f64, t833);
    let t1300 = t447 * t1299;
    let t1301 = t1300 * t637;
    let t1305 = t237 * t318 * t451;
    (t1295, t1296, t1300, t1301, t1305)
}
