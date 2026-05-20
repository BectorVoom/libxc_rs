//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1872;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta554<F: Float>(t5697: F, t94429: F, t5701: F, t27928: F, t9775: F, t13775: F, t25986: F, t2661: F, t25978: F, t5614: F, t5622: F, t94443: F, t13769: F, t240: F, t7269: F, t13760: F, t25972: F, t5609: F, t7028: F, t9845: F, t1889: F, t94545: F, t13846: F, t13877: F, t7021: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98128, t98130, t98141, t98144, t98146, t98148) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1872::<F>(t5697, t94429, t5701, t27928, t9775, t13775, t25986, t2661, t25978, t5614, t5622, t94443);
        let (t98152, t98156, t98161, t98165, t98168) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1873::<F>(t13769, t240, t2661, t7269, t13760, t25972, t5609, t7028, t9845, t1889, t94545, t13846, t13877, t7021);
    (t98128, t98130, t98141, t98144, t98146, t98148, t98152, t98156, t98161, t98165, t98168)
}
