//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3532/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3532<F: Float>(t6244: F, t905: F, t11774: F, t4782: F, t53391: F, t1011: F, t15993: F, t18909: F, t11792: F, t15599: F, t15610: F, t15688: F, t15696: F, t15964: F, t15968: F, t16096: F, t16103: F, t16183: F, t1664: F, t3091: F, t3092: F, t3117: F, t3154: F, t43049: F, t4892: F, t4893: F, t54836: F, t54841: F, t54849: F, t54869: F, t55331: F, t6096: F, t6278: F, t66777: F) -> F {
    let t66966 = t6244 * t905;
    let t66972 = t11774 * t53391 * t4782;
    let t66981 = t1011 * t15993 * t18909;
    let t66997 = F::cast_from(0.34299214494455789578e-2_f64) * t43049 * t15688 * t1664 * t15610 + F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t3117 * t4893 * t3154 * t16183 - F::cast_from(0.17149607247227894789e-2_f64) * t55331 * t3092 * t66966 * t16096 - F::cast_from(0.3811023832717309953e-3_f64) * t66972 + F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t15696 * t15964 - F::cast_from(0.20325460441158986416e-2_f64) * t54836 + F::cast_from(0.28582678745379824648e-3_f64) * t54841 - F::cast_from(0.7622047665434619906e-3_f64) * t54849 - t66981 / F::cast_from(54.0_f64) - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t66777 * t16103 + F::cast_from(0.28582678745379824648e-3_f64) * t54869 + F::cast_from(0.22866142996303859718e-2_f64) * t11792 * t6278 - F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t3092 * t6096 * t15599 - F::cast_from(0.57165357490759649296e-3_f64) * t4892 * t3092 * t6096 * t15968;
    t66997
}
