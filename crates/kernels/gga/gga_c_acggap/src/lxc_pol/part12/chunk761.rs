//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 761/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk761<F: Float>(t2176: F, t872: F, t1264: F, t2147: F, t633: F, t7311: F, t7327: F, t7372: F, t7375: F, t7378: F, t7313: F, t7316: F, t7318: F, t7330: F, t7333: F, t7340: F, t7344: F, t7349: F, t7354: F, t7358: F, t7362: F, t7366: F, t7368: F, t7383: F) -> (F, F, F, F, F, F, F, F) {
    let t8123 = t2176 * t872;
    let t8126 = t2147 * t633 * t1264;
    let t8129 = F::cast_from(0.1324375e0_f64) * t7311;
    let t8133 = F::cast_from(0.7640625e-2_f64) * t7327;
    let t8144 = F::cast_from(0.22675591804667994221e-1_f64) * t7372;
    let t8145 = F::cast_from(0.80031500487063509014e-2_f64) * t7375;
    let t8146 = F::cast_from(0.85748036236139473944e-3_f64) * t7378;
    let t8148 = t8129 - t7313 / F::cast_from(48.0_f64) + F::cast_from(11.0_f64) / F::cast_from(96.0_f64) * t7316 + F::cast_from(11.0_f64) / F::cast_from(288.0_f64) * t7318 - t8133 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t7330 + t7333 / F::cast_from(8.0_f64) + F::cast_from(0.21437009059034868486e-2_f64) * t7340 + F::cast_from(0.85748036236139473944e-3_f64) * t7344 + F::cast_from(0.42874018118069736972e-3_f64) * t7349 - F::cast_from(0.18868855373762491241e-2_f64) * t7354 - F::cast_from(0.31448092289604152068e-2_f64) * t7358 + F::cast_from(0.12579236915841660828e-2_f64) * t7362 - F::cast_from(0.62896184579208304138e-3_f64) * t7366 + F::cast_from(0.17149607247227894789e-2_f64) * t7368 + t8144 - t8145 + t8146 - t7383 / F::cast_from(16.0_f64);
    (t8123, t8126, t8129, t8133, t8144, t8145, t8146, t8148)
}
