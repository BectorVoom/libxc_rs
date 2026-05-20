//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2988/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2988<F: Float>(t12021: F, t4820: F, t11998: F, t15822: F, t1042: F, t11151: F, t11774: F, t15584: F, t15586: F, t15599: F, t15907: F, t15950: F, t16081: F, t16082: F, t16170: F, t1671: F, t3097: F, t3117: F, t3127: F, t3164: F, t42155: F, t42690: F, t42970: F, t4786: F, t4873: F, t54469: F, t54471: F, t54474: F, t54479: F) -> F {
    let t54490 = t12021 * t4820;
    let t54492 = t15822 * t11998;
    let t54495 = -F::cast_from(0.85748036236139473944e-3_f64) * t42155 * t15586 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15584 * t15950 * t4786 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15584 * t4873 * t15599 + F::cast_from(0.17149607247227894789e-2_f64) * t54469 - F::cast_from(0.45732285992607719436e-2_f64) * t54471 * t3097 - F::cast_from(0.64311027177104605458e-3_f64) * t42690 * t3117 * t15907 * t54474 + F::cast_from(0.38586616306262763275e-2_f64) * t16081 * t3117 * t54479 * t16082 + F::cast_from(0.14291339372689912324e-2_f64) * t3127 * t1042 * t16170 * t11151 - F::cast_from(0.34299214494455789577e-2_f64) * t42970 * t1671 + F::cast_from(0.42874018118069736972e-3_f64) * t54490 + F::cast_from(0.34299214494455789577e-2_f64) * t54492 * t3164;
    t54495
}
