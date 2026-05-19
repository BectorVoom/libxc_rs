//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1259/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1259<F: Float>(t1020: F, t1129: F, t1131: F, t1133: F, t1135: F, t12296: F, t2410: F, t2412: F, t2958: F, t3524: F, t3526: F, t3530: F, t3534: F, t3538: F, t3747: F, t3749: F, t3753: F, t3757: F, t3761: F, t9711: F) -> F {
    let t44746 = F::cast_from(0.1469548921044e3_f64) * t3749 * t2412 + F::cast_from(0.734774460522e2_f64) * t1129 * t9711 - F::cast_from(0.22988522834472e3_f64) * t3753 * t2412 - F::cast_from(0.11494261417236e3_f64) * t1131 * t9711 + F::cast_from(0.12405227240928e3_f64) * t3757 * t2412 + F::cast_from(0.6202613620464e2_f64) * t1133 * t9711 - F::cast_from(0.2177652951264e2_f64) * t3761 * t2412 - F::cast_from(0.1088826475632e2_f64) * t1135 * t9711 + F::cast_from(0.734774460522e2_f64) * t3524 * t2958 + F::cast_from(0.734774460522e2_f64) * t3526 * t2958 - F::cast_from(0.11494261417236e3_f64) * t3530 * t2958 + F::cast_from(0.6202613620464e2_f64) * t3534 * t2958 - F::cast_from(0.1088826475632e2_f64) * t3538 * t2958 - F::cast_from(0.18428227254588e2_f64) * t3747 * t2410 - F::cast_from(0.18428227254588e2_f64) * t12296 * t1020;
    t44746
}
