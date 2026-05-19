//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1009/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1009<F: Float>(t2124: F, t9318: F, t3295: F, t11697: F, t11728: F, t11730: F, t12138: F, t12477: F, t12480: F, t12482: F, t12487: F, t12490: F, t12493: F, t12496: F, t12499: F, t12501: F, t12504: F) -> (F, F) {
    let t12506 = t2124 * t9318;
    let t12507 = t3295 * t12506;
    let t12509 = F::cast_from(0.47609969197673950972e-2_f64) * t11697 + t12138 + F::cast_from(0.86682217400542685632e-1_f64) * t12477 + F::cast_from(0.2600466522016280569e0_f64) * t12480 + F::cast_from(0.10975748638225852664e0_f64) * t12482 + F::cast_from(0.69345773920434148506e0_f64) * t11728 + F::cast_from(0.25610080155860322884e0_f64) * t11730 + F::cast_from(0.86682217400542685632e-1_f64) * t12487 + F::cast_from(0.5200933044032561138e0_f64) * t12490 - F::cast_from(0.43663693315433241792e-2_f64) * t12493 - F::cast_from(0.13099107994629972538e-1_f64) * t12496 - F::cast_from(0.43341108700271342816e-1_f64) * t12499 - F::cast_from(0.2600466522016280569e0_f64) * t12501 - F::cast_from(0.13002332610081402845e0_f64) * t12504 + F::cast_from(0.54878743191129263322e-1_f64) * t12507;
    (t12506, t12509)
}
