//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1385/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1385<F: Float>(t1429: F, t39501: F, t544: F, t9989: F, t555: F, t4003: F, t1433: F, t39545: F, t546: F, t685: F, t39552: F, t557: F) -> (F, F, F, F, F, F) {
    let t46412 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1429;
    let t46475 = F::new(1.0) / t9989 / t544;
    let t46476 = t46475 * t555;
    let t46478 = t4003 * t4003;
    let t46515 = F::cast_from(0.65457331274007190912e-5_f64) * t39545 * t546 * t1433 * t685;
    let t46518 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t557;
    (t46412, t46475, t46476, t46478, t46515, t46518)
}
