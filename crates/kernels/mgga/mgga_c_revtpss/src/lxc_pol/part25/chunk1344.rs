//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1344/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1344<F: Float>(t2014: F, t2034: F, t46304: F, t2327: F, t6982: F, t1936: F, t46126: F, t49851: F, t10416: F, t7002: F, t49693: F, t13435: F) -> (F, F, F, F, F, F, F) {
    let t94944 = t2014 * t2034 * t46304;
    let t94947 = t6982 * t2327;
    let t94956 = F::new(2.0) * t46126 * t1936;
    let t94958 = F::new(6.0) * t49851 * t1936;
    let t94960 = F::new(6.0) * t10416 * t7002;
    let t94962 = F::new(6.0) * t49693 * t1936;
    let t94964 = F::new(12.0) * t13435 * t7002;
    (t94944, t94947, t94956, t94958, t94960, t94962, t94964)
}
