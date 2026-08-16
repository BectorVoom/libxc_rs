//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1021/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1021<F: Float>(t1020: F, t3524: F, t1129: F, t1131: F, t1133: F, t1135: F, t12285: F, t12286: F, t12288: F, t12292: F, t12294: F, t12296: F, t12298: F, t12300: F, t12302: F, t2412: F, t343: F) -> F {
    let t12305 = t3524 * t1020;
    let t12307 = F::cast_from(0.734774460522e2_f64) * t1129 * t2412 - F::cast_from(0.11494261417236e3_f64) * t1131 * t2412 + F::cast_from(0.6202613620464e2_f64) * t1133 * t2412 - F::cast_from(0.1088826475632e2_f64) * t1135 * t2412 - F::cast_from(0.957855118103e1_f64) * t12286 + F::cast_from(0.3101306810232e1_f64) * t12288 - F::cast_from(0.362942158544e0_f64) * t343 * t12285 - F::cast_from(0.8704e0_f64) * t12292 - F::cast_from(0.8704e0_f64) * t12294 - F::cast_from(0.8704e0_f64) * t12296 - F::cast_from(0.8704e0_f64) * t12298 - F::cast_from(0.4607056813647e1_f64) * t12300 + F::cast_from(0.122462410087e2_f64) * t12302 - F::cast_from(0.64e0_f64) * t12285 - F::cast_from(0.9214113627294e1_f64) * t12305;
    t12307
}
