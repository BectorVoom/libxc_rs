//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1897;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta573<F: Float>(t102100: F, t25944: F, t25950: F, t28845: F, t28780: F, t94886: F, t28889: F, t686: F, t72: F, t7284: F, t10073: F, t1903: F, t2102: F, t25929: F, t28837: F, t3920: F, t1358: F, t212: F, t28888: F, t689: F, t25898: F, t8099: F, t94849: F, t26277: F, t97916: F, t97799: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t102101, t102104, t102113, t102115, t102117, t102120) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1897::<F>(t102100, t25944, t25950, t28845, t28780, t94886, t28889, t686, t72, t7284, t10073, t1903, t2102, t25929);
        let (t102122, t102129, t102131, t102133, t102135) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1898::<F>(t28837, t3920, t1358, t212, t28888, t689, t25898, t8099, t94849, t26277, t97916, t97799);
    (t102101, t102104, t102113, t102115, t102117, t102120, t102122, t102129, t102131, t102133, t102135)
}
