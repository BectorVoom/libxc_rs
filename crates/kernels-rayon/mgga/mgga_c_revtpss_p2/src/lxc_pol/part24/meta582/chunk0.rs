//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1812/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1812(t6861: f64, t9994: f64, t13789: f64, t13804: f64, t1390: f64, t1410: f64, t1414: f64, t1868: f64, t1882: f64, t1883: f64, t22046: f64, t22079: f64, t23037: f64, t3934: f64, t3936: f64, t48518: f64, t5671: f64, t5673: f64, t6862: f64, t6869: f64, t73929: f64, t73953: f64, t74017: f64, t74024: f64, t828: f64, t85553: f64, t85563: f64, t85638: f64, t85659: f64, t85705: f64, t85735: f64, t85741: f64, t85752: f64, t91826: f64, t9993: f64) -> (f64, f64, f64) {
    let t91921 = t6861 * t6861;
    let t91922 = t91921 * t9994;
    let t91927 = 0.34299214494455789577e-2_f64 * t3934 * t3936 * t85563 * t1883 + 0.77173232612525526552e-2_f64 * t5671 * t5673 * t22079 * t6862 - 0.20579528696673473746e-1_f64 * t5671 * t13789 * t23037 * t1868 * t1882 + 0.10289764348336736873e-1_f64 * t3934 * t13789 * t85659 * t6869 - 0.85748036236139473944e-3_f64 * t1410 * t1414 * t828 * t91826 + 0.68026775414003982664e-1_f64 * t73929 + 0.81312004494856525159e-3_f64 * t73953 + 0.34299214494455789577e-2_f64 * t3934 * t3936 * t85553 * t6869 + 0.12004725073059526352e-1_f64 * t85705 + 0.91464571985215438873e-3_f64 * t74017 + 0.36585828794086175548e-2_f64 * t74024 - 0.77173232612525526552e-2_f64 * t13804 * t5673 * t22046 * t85638 + 0.60984003371142393869e-3_f64 * t85735 - 0.34299214494455789577e-3_f64 * t85741 + 0.15117061203111996148e0_f64 * t48518 + 0.96037800584476210818e-1_f64 * t85752 - 0.77173232612525526552e-2_f64 * t9993 * t1390 * t828 * t91922;
    (t91921, t91922, t91927)
}
