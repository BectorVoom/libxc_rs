//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta409<F: Float>(t10208: F, t104: F, t69: F, t2339: F, t2681: F, t64: F, t10207: F, t111: F, t116: F, t13424: F, t1501: F, t2371: F, t4245: F, t670: F, t1518: F, t2319: F, t4292: F, t648: F, t13514: F, t94: F, t1513: F, t2340: F, t4287: F, t665: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t36308, t36315, t46089, t46157, t49686, t75485) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1489::<F>(t10208, t104, t69, t2339, t2681, t64, t10207, t111, t116, t13424, t1501, t2371);
        let (t75667, t98484, t98487, t98535, t101457, t101460) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1490::<F>(t4245, t670, t1518, t2319, t4292, t648, t13514, t94, t1513, t2340, t4287, t665);
    (t36308, t36315, t46089, t46157, t49686, t75485, t75667, t98484, t98487, t98535, t101457, t101460)
}
