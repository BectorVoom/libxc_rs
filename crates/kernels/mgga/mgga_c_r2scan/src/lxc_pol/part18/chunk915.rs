//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 915/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk915<F: Float>(t12495: F, t6093: F, t261: F, t3217: F, t3299: F, t11748: F, t3594: F, t3223: F, t3304: F, t2124: F, t9318: F, t3295: F, t11697: F, t11728: F, t11730: F, t12138: F, t12477: F, t12480: F, t12482: F, t12487: F, t12490: F, t12493: F) -> (F, F, F, F) {
    let t12496 = t6093 * t12495;
    let t12498 = t261 * t3217;
    let t12499 = t3299 * t12498;
    let t12501 = t11748 * t3594;
    let t12503 = t261 * t3223;
    let t12504 = t3304 * t12503;
    let t12506 = t2124 * t9318;
    let t12507 = t3295 * t12506;
    let t12509 = 0.47609969197673950972e-2 * t11697 + t12138 + 0.86682217400542685632e-1 * t12477 + 0.2600466522016280569e0 * t12480 + 0.10975748638225852664e0 * t12482 + 0.69345773920434148506e0 * t11728 + 0.25610080155860322884e0 * t11730 + 0.86682217400542685632e-1 * t12487 + 0.5200933044032561138e0 * t12490 - 0.43663693315433241792e-2 * t12493 - 0.13099107994629972538e-1 * t12496 - 0.43341108700271342816e-1 * t12499 - 0.2600466522016280569e0 * t12501 - 0.13002332610081402845e0 * t12504 + 0.54878743191129263322e-1 * t12507;
    (t12498, t12503, t12506, t12509)
}
