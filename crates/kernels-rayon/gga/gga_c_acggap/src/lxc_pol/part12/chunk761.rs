//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 761/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk761(t2176: f64, t872: f64, t1264: f64, t2147: f64, t633: f64, t7311: f64, t7327: f64, t7372: f64, t7375: f64, t7378: f64, t7313: f64, t7316: f64, t7318: f64, t7330: f64, t7333: f64, t7340: f64, t7344: f64, t7349: f64, t7354: f64, t7358: f64, t7362: f64, t7366: f64, t7368: f64, t7383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8123 = t2176 * t872;
    let t8126 = t2147 * t633 * t1264;
    let t8129 = 0.1324375e0_f64 * t7311;
    let t8133 = 0.7640625e-2_f64 * t7327;
    let t8144 = 0.22675591804667994221e-1_f64 * t7372;
    let t8145 = 0.80031500487063509014e-2_f64 * t7375;
    let t8146 = 0.85748036236139473944e-3_f64 * t7378;
    let t8148 = t8129 - t7313 / 48.0_f64 + 11.0_f64 / 96.0_f64 * t7316 + 11.0_f64 / 288.0_f64 * t7318 - t8133 + 7.0_f64 / 36.0_f64 * t7330 + t7333 / 8.0_f64 + 0.21437009059034868486e-2_f64 * t7340 + 0.85748036236139473944e-3_f64 * t7344 + 0.42874018118069736972e-3_f64 * t7349 - 0.18868855373762491241e-2_f64 * t7354 - 0.31448092289604152068e-2_f64 * t7358 + 0.12579236915841660828e-2_f64 * t7362 - 0.62896184579208304138e-3_f64 * t7366 + 0.17149607247227894789e-2_f64 * t7368 + t8144 - t8145 + t8146 - t7383 / 16.0_f64;
    (t8123, t8126, t8129, t8133, t8144, t8145, t8146, t8148)
}
