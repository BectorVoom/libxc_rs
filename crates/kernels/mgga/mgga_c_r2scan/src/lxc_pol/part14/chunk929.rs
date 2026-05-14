//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 929/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk929<F: Float>(t12285: F, t339: F, t341: F, t1127: F, t2410: F, t1020: F, t3522: F, t3745: F, t839: F, t333: F, t335: F, t337: F, t3524: F, t1129: F, t1131: F, t1133: F, t1135: F, t2412: F, t343: F) -> (F, F, F, F, F, F) {
    let t12286 = t339 * t12285;
    let t12288 = t341 * t12285;
    let t12292 = t2410 * t1127;
    let t12294 = t1020 * t3522;
    let t12296 = t839 * t3745;
    let t12298 = t333 * t12285;
    let t12300 = t335 * t12285;
    let t12302 = t337 * t12285;
    let t12305 = t3524 * t1020;
    let t12307 = 0.734774460522e2 * t1129 * t2412 - 0.11494261417236e3 * t1131 * t2412 + 0.6202613620464e2 * t1133 * t2412 - 0.1088826475632e2 * t1135 * t2412 - 0.957855118103e1 * t12286 + 0.3101306810232e1 * t12288 - 0.362942158544e0 * t343 * t12285 - 0.8704e0 * t12292 - 0.8704e0 * t12294 - 0.8704e0 * t12296 - 0.8704e0 * t12298 - 0.4607056813647e1 * t12300 + 0.122462410087e2 * t12302 - 0.64e0 * t12285 - 0.9214113627294e1 * t12305;
    (t12286, t12288, t12298, t12300, t12302, t12307)
}
