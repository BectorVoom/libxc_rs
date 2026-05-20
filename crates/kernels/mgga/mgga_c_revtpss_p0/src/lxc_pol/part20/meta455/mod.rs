//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1738;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta455<F: Float>(t3855: F, t3857: F, t40082: F, t512: F, t520: F, t1333: F, t9410: F, t1320: F, t9428: F, t1331: F, t9413: F, t3853: F, t3863: F, t9561: F, t9554: F, t39483: F, t39520: F, t39528: F, t39531: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46968, t46970, t46972, t46974, t46976, t46978, t46979) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1738::<F>(t3855, t3857, t40082, t512, t520, t1333, t9410, t1320, t9428, t1331, t9413, t3853, t3863);
        let (t46980, t46982, t46984, t46985) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1739::<F>(t46979, t1320, t9561, t9554, t39483, t39520, t39528, t39531, t46968, t46970, t46972, t46974, t46976, t46978);
    (t46968, t46970, t46972, t46974, t46976, t46978, t46980, t46982, t46984, t46985)
}
