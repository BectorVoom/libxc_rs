//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3532/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3532(t6244: f64, t905: f64, t11774: f64, t4782: f64, t53391: f64, t1011: f64, t15993: f64, t18909: f64, t11792: f64, t15599: f64, t15610: f64, t15688: f64, t15696: f64, t15964: f64, t15968: f64, t16096: f64, t16103: f64, t16183: f64, t1664: f64, t3091: f64, t3092: f64, t3117: f64, t3154: f64, t43049: f64, t4892: f64, t4893: f64, t54836: f64, t54841: f64, t54849: f64, t54869: f64, t55331: f64, t6096: f64, t6278: f64, t66777: f64) -> f64 {
    let t66966 = t6244 * t905;
    let t66972 = t11774 * t53391 * t4782;
    let t66981 = t1011 * t15993 * t18909;
    let t66997 = 0.34299214494455789578e-2_f64 * t43049 * t15688 * t1664 * t15610 + 0.85748036236139473944e-3_f64 * t4892 * t3117 * t4893 * t3154 * t16183 - 0.17149607247227894789e-2_f64 * t55331 * t3092 * t66966 * t16096 - 0.3811023832717309953e-3_f64 * t66972 + 0.57165357490759649296e-3_f64 * t11774 * t15696 * t15964 - 0.20325460441158986416e-2_f64 * t54836 + 0.28582678745379824648e-3_f64 * t54841 - 0.7622047665434619906e-3_f64 * t54849 - t66981 / 54.0_f64 - 0.28582678745379824648e-3_f64 * t11774 * t66777 * t16103 + 0.28582678745379824648e-3_f64 * t54869 + 0.22866142996303859718e-2_f64 * t11792 * t6278 - 0.28582678745379824648e-3_f64 * t3091 * t3092 * t6096 * t15599 - 0.57165357490759649296e-3_f64 * t4892 * t3092 * t6096 * t15968;
    t66997
}
