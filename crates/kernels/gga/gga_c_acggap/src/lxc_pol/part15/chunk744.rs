//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 744/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk744<F: Float>(t2217: F, t315: F, t323: F, t2176: F, t872: F, t7311: F, t7327: F, t7372: F, t7375: F, t7378: F, t7462: F, t7515: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8114 = t315 * t2217;
    let t8115 = t8114 * t323;
    let t8123 = t2176 * t872;
    let t8129 = F::cast_from(0.1324375e0_f64) * t7311;
    let t8133 = F::cast_from(0.7640625e-2_f64) * t7327;
    let t8144 = F::cast_from(0.22675591804667994221e-1_f64) * t7372;
    let t8145 = F::cast_from(0.80031500487063509014e-2_f64) * t7375;
    let t8146 = F::cast_from(0.85748036236139473944e-3_f64) * t7378;
    let t8171 = F::cast_from(0.28582678745379824648e-3_f64) * t7462;
    let t8184 = F::cast_from(0.85748036236139473944e-3_f64) * t7515;
    (t8114, t8115, t8123, t8129, t8133, t8144, t8145, t8146, t8171, t8184)
}
