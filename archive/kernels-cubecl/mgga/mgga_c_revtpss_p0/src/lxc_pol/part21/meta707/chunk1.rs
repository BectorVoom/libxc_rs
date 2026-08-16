//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2535/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2535<F: Float>(t3923: F, t68: F, t10139: F, t281: F, t543: F, t1433: F, t39545: F, t546: F, t685: F, t39552: F, t557: F, t10103: F, t1432: F, t2470: F) -> (F, F, F, F, F) {
    let t46507 = t68 * t3923;
    let t46510 = t10139 * t281 * t46507 * t543;
    let t46515 = F::cast_from(0.65457331274007190912e-5_f64) * t39545 * t546 * t1433 * t685;
    let t46518 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t557;
    let t46520 = t1432 * t10103 * t2470;
    (t46507, t46510, t46515, t46518, t46520)
}
