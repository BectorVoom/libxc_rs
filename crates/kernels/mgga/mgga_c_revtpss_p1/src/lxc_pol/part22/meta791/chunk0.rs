//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2883/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2883<F: Float>(t10039: F, t2439: F, t2777: F, t1429: F, t39501: F, t4056: F, t9994: F, t10014: F, t10136: F, t215: F, t3923: F, t268: F, t4101: F, t543: F) -> (F, F, F, F, F, F) {
    let t46401 = t2439 * t2777 * t10039;
    let t46412 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1429;
    let t46416 = t9994 * t4056;
    let t46443 = t10014 * t10136;
    let t46445 = t215 * t3923;
    let t46448 = t4101 * t268 * t46445 * t543;
    (t46401, t46412, t46416, t46443, t46445, t46448)
}
