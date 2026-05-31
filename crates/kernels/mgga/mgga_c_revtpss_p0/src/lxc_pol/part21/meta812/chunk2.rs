//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2971/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2971<F: Float>(t11773: F, t3278: F, t11687: F, t11774: F, t12021: F, t15584: F, t15689: F, t15691: F, t15700: F, t15701: F, t15703: F, t15809: F, t16009: F, t16013: F, t1671: F, t3095: F, t3241: F, t42235: F, t42425: F, t42699: F, t42710: F, t4786: F, t4869: F, t4875: F, t53846: F) -> (F, F) {
    let t54166 = t3278 * t11773;
    let t54176 = F::cast_from(0.57165357490759649295e-3_f64) * t42699 - F::cast_from(0.14481890564325777822e-1_f64) * t42425 * t4875 + F::cast_from(0.21437009059034868486e-3_f64) * t42235 * t1671 + F::cast_from(0.64311027177104605458e-3_f64) * t12021 * t4869 - F::cast_from(0.95275595817932748827e-4_f64) * t42710 - F::cast_from(0.42874018118069736972e-3_f64) * t15689 * t15691 * t11687 * t3095 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15584 * t15809 * t4786 - F::cast_from(0.17149607247227894789e-2_f64) * t54166 * t15703 - F::cast_from(0.17149607247227894789e-2_f64) * t15700 * t15701 * t53846 - t3241 * t16009 / F::cast_from(27.0_f64) - F::cast_from(7.0_f64) / F::cast_from(81.0_f64) * t3241 * t16013;
    (t54166, t54176)
}
