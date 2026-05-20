//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3004/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3004<F: Float>(t42415: F, t4890: F, t1062: F, t42261: F, t11913: F, t15719: F, t15850: F, t15975: F, t16049: F, t3101: F, t3299: F, t3317: F, t43029: F, t43032: F, t43035: F, t43038: F, t43121: F, t4834: F, t4896: F, t4902: F, t4912: F) -> F {
    let t54885 = t42415 * t4890;
    let t54899 = t42261 * t1062;
    let t54904 = -F::cast_from(0.64311027177104605458e-3_f64) * t43038 * t4912 - t43029 / F::new(144.0) + t43032 / F::new(216.0) + F::cast_from(0.43445671692977333464e-1_f64) * t3299 * t54885 * t4896 - F::cast_from(0.21722835846488666732e-1_f64) * t3317 * t54885 * t4902 + F::cast_from(0.68598428988911579154e-2_f64) * t43121 * t4912 - F::cast_from(0.42874018118069736972e-3_f64) * t43035 - F::cast_from(0.85748036236139473944e-3_f64) * t15850 * t3101 - F::cast_from(0.14291339372689912324e-2_f64) * t4834 * t11913 - F::cast_from(0.38586616306262763275e-2_f64) * t54899 * t15719 + F::cast_from(0.22866142996303859718e-2_f64) * t16049 * t15975;
    t54904
}
