//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1693/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1693<F: Float>(t3094: F, t5825: F, t1668: F, t23598: F, t1045: F, t1063: F, t11660: F, t11703: F, t15926: F, t16081: F, t19450: F, t19501: F, t19611: F, t19741: F, t23630: F, t23936: F, t23994: F, t247: F, t3091: F, t3092: F, t3115: F, t3117: F, t3182: F, t42215: F, t4834: F, t4892: F, t4899: F, t53326: F, t5819: F, t6273: F, t67551: F, t78863: F, t80358: F, t88120: F, t88844: F) -> (F, F) {
    let t88857 = t3094 * t5825;
    let t88885 = t23598 * t1668;
    let t88898 = F::cast_from(0.28582678745379824648e-2_f64) * t4892 * t11703 * t19501 * t42215 * t5819 - F::cast_from(0.85748036236139473944e-3_f64) * t4899 * t3092 * t19501 * t88857 - F::cast_from(0.14291339372689912324e-2_f64) * t4899 * t11703 * t19501 * t88844 + F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t3092 * t19611 * t88857 + F::cast_from(0.17149607247227894789e-2_f64) * t4892 * t3092 * t19501 * t11660 * t5825 + F::cast_from(0.22866142996303859719e-2_f64) * t78863 + F::cast_from(0.2540682555144873302e-3_f64) * t53326 + F::cast_from(0.77173232612525526552e-2_f64) * t16081 * t3117 * t19450 * t80358 - F::cast_from(0.25724410870841842184e-2_f64) * t19741 * t23936 - F::cast_from(0.25724410870841842184e-2_f64) * t15926 * t23994 - F::cast_from(0.85748036236139473944e-3_f64) * t3115 * t3117 * t88885 * t1045 - F::cast_from(0.25724410870841842184e-2_f64) * t67551 * t6273 + F::cast_from(0.34299214494455789577e-2_f64) * t4834 * t23630 + F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t247 * t3182 * t88120;
    (t88885, t88898)
}
