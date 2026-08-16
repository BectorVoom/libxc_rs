//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1693/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1693(t3094: f64, t5825: f64, t1668: f64, t23598: f64, t1045: f64, t1063: f64, t11660: f64, t11703: f64, t15926: f64, t16081: f64, t19450: f64, t19501: f64, t19611: f64, t19741: f64, t23630: f64, t23936: f64, t23994: f64, t247: f64, t3091: f64, t3092: f64, t3115: f64, t3117: f64, t3182: f64, t42215: f64, t4834: f64, t4892: f64, t4899: f64, t53326: f64, t5819: f64, t6273: f64, t67551: f64, t78863: f64, t80358: f64, t88120: f64, t88844: f64) -> (f64, f64) {
    let t88857 = t3094 * t5825;
    let t88885 = t23598 * t1668;
    let t88898 = 0.28582678745379824648e-2_f64 * t4892 * t11703 * t19501 * t42215 * t5819 - 0.85748036236139473944e-3_f64 * t4899 * t3092 * t19501 * t88857 - 0.14291339372689912324e-2_f64 * t4899 * t11703 * t19501 * t88844 + 0.85748036236139473944e-3_f64 * t3091 * t3092 * t19611 * t88857 + 0.17149607247227894789e-2_f64 * t4892 * t3092 * t19501 * t11660 * t5825 + 0.22866142996303859719e-2_f64 * t78863 + 0.2540682555144873302e-3_f64 * t53326 + 0.77173232612525526552e-2_f64 * t16081 * t3117 * t19450 * t80358 - 0.25724410870841842184e-2_f64 * t19741 * t23936 - 0.25724410870841842184e-2_f64 * t15926 * t23994 - 0.85748036236139473944e-3_f64 * t3115 * t3117 * t88885 * t1045 - 0.25724410870841842184e-2_f64 * t67551 * t6273 + 0.34299214494455789577e-2_f64 * t4834 * t23630 + 0.85748036236139473944e-2_f64 * t1063 * t247 * t3182 * t88120;
    (t88885, t88898)
}
