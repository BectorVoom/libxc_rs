//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1728/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1728<F: Float>(t2661: F, t4003: F, t46610: F, t9934: F, t40735: F, t535: F, t235: F, t5744: F, t2453: F, t9794: F, t9935: F, t1389: F, t268: F) -> (F, F, F, F) {
    let t46797 = t2661 * t9934 * t46610 * t4003;
    let t46800 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t40735 * t535;
    let t46801 = t5744 * t235;
    let t46802 = t2453 * t46801;
    let t46804 = t46802 * t9794 * t9935;
    let t46808 = t1389 * t268;
    (t46797, t46800, t46804, t46808)
}
