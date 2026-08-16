//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2502/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2502<F: Float>(t1512: F, t41354: F, t13198: F, t2697: F, t13302: F, t9638: F, t13306: F, t13248: F, t13258: F, t1484: F, t2631: F, t4233: F, t828: F) -> (F, F, F, F, F, F, F) {
    let t46960 = t41354 * t1512;
    let t46962 = t2697 * t13198;
    let t46974 = t9638 * t13302;
    let t46980 = t9638 * t13306;
    let t46998 = t13258 * t13248;
    let t47012 = t1484 * t2631;
    let t47017 = t4233 * t828;
    (t46960, t46962, t46974, t46980, t46998, t47012, t47017)
}
