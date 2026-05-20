//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3542/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3542<F: Float>(t15745: F, t4845: F, t1011: F, t1012: F, t1045: F, t11774: F, t11927: F, t15149: F, t15651: F, t15656: F, t15691: F, t15696: F, t15700: F, t15958: F, t1665: F, t19620: F, t20089: F, t3117: F, t3236: F, t4782: F, t4854: F, t4858: F, t53866: F, t54384: F, t54818: F, t55104: F, t55148: F, t55150: F, t60717: F) -> F {
    let t67301 = t15745 * t4845;
    let t67318 = F::cast_from(0.17149607247227894789e-2_f64) * t11927 * t3117 * t20089 * t19620 - F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t54818 * t4782 - F::cast_from(0.3811023832717309953e-3_f64) * t55104 - t1011 * t1012 * t3236 * t60717 / F::new(72.0) - F::cast_from(0.14481890564325777821e-1_f64) * t54384 * t1665 + F::cast_from(0.30488190661738479624e-2_f64) * t67301 - F::cast_from(0.42874018118069736972e-3_f64) * t53866 * t1665 - F::cast_from(0.85748036236139473944e-3_f64) * t15656 * t4854 - F::cast_from(0.42874018118069736972e-3_f64) * t4858 * t15651 - F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t15696 * t15958 - F::cast_from(0.57165357490759649296e-3_f64) * t15700 * t15691 * t1045 * t15149 - F::cast_from(0.28582678745379824648e-3_f64) * t55148 + F::cast_from(0.30488190661738479624e-2_f64) * t55150;
    t67318
}
