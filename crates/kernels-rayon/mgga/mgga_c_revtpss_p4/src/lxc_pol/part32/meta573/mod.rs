//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1897;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta573(t102100: f64, t25944: f64, t25950: f64, t28845: f64, t28780: f64, t94886: f64, t28889: f64, t686: f64, t72: f64, t7284: f64, t10073: f64, t1903: f64, t2102: f64, t25929: f64, t28837: f64, t3920: f64, t1358: f64, t212: f64, t28888: f64, t689: f64, t25898: f64, t8099: f64, t94849: f64, t26277: f64, t97916: f64, t97799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102101, t102104, t102113, t102115, t102117, t102120) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1897(t102100, t25944, t25950, t28845, t28780, t94886, t28889, t686, t72, t7284, t10073, t1903, t2102, t25929);
        let (t102122, t102129, t102131, t102133, t102135) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1898(t28837, t3920, t1358, t212, t28888, t689, t25898, t8099, t94849, t26277, t97916, t97799);
    (t102101, t102104, t102113, t102115, t102117, t102120, t102122, t102129, t102131, t102133, t102135)
}
