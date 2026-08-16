//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1812/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1812<F: Float>(t6861: F, t9994: F, t13789: F, t13804: F, t1390: F, t1410: F, t1414: F, t1868: F, t1882: F, t1883: F, t22046: F, t22079: F, t23037: F, t3934: F, t3936: F, t48518: F, t5671: F, t5673: F, t6862: F, t6869: F, t73929: F, t73953: F, t74017: F, t74024: F, t828: F, t85553: F, t85563: F, t85638: F, t85659: F, t85705: F, t85735: F, t85741: F, t85752: F, t91826: F, t9993: F) -> (F, F, F) {
    let t91921 = t6861 * t6861;
    let t91922 = t91921 * t9994;
    let t91927 = F::cast_from(0.34299214494455789577e-2_f64) * t3934 * t3936 * t85563 * t1883 + F::cast_from(0.77173232612525526552e-2_f64) * t5671 * t5673 * t22079 * t6862 - F::cast_from(0.20579528696673473746e-1_f64) * t5671 * t13789 * t23037 * t1868 * t1882 + F::cast_from(0.10289764348336736873e-1_f64) * t3934 * t13789 * t85659 * t6869 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t1414 * t828 * t91826 + F::cast_from(0.68026775414003982664e-1_f64) * t73929 + F::cast_from(0.81312004494856525159e-3_f64) * t73953 + F::cast_from(0.34299214494455789577e-2_f64) * t3934 * t3936 * t85553 * t6869 + F::cast_from(0.12004725073059526352e-1_f64) * t85705 + F::cast_from(0.91464571985215438873e-3_f64) * t74017 + F::cast_from(0.36585828794086175548e-2_f64) * t74024 - F::cast_from(0.77173232612525526552e-2_f64) * t13804 * t5673 * t22046 * t85638 + F::cast_from(0.60984003371142393869e-3_f64) * t85735 - F::cast_from(0.34299214494455789577e-3_f64) * t85741 + F::cast_from(0.15117061203111996148e0_f64) * t48518 + F::cast_from(0.96037800584476210818e-1_f64) * t85752 - F::cast_from(0.77173232612525526552e-2_f64) * t9993 * t1390 * t828 * t91922;
    (t91921, t91922, t91927)
}
