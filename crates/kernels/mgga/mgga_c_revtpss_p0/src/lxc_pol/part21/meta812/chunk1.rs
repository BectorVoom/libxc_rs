//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2970/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2970<F: Float>(t11921: F, t15837: F, t247: F, t4837: F, t11267: F, t4878: F, t11263: F, t4879: F, t1047: F, t1068: F, t11714: F, t11875: F, t15606: F, t3116: F, t3117: F, t42830: F, t4831: F, t4893: F, t54112: F, t54118: F, t54123: F, t54127: F, t54130: F, t54137: F) -> F {
    let t54142 = t4837 * t247 * t11921 * t15837;
    let t54144 = t4878 * t11267;
    let t54147 = t4879 * t11263;
    let t54148 = F::cast_from(0.14291339372689912324e-3_f64) * t54147;
    let t54149 = F::cast_from(0.12862205435420921092e-2_f64) * t4837 * t247 * t3116 * t54112 + F::new(5.0) / F::new(3888.0) * t54118 + t54123 - t54127 + F::cast_from(0.12862205435420921092e-2_f64) * t42830 * t15606 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t4893 * t54130 - F::cast_from(0.45732285992607719436e-2_f64) * t11714 * t4831 + F::cast_from(0.42874018118069736972e-3_f64) * t54137 * t1068 + F::cast_from(0.85748036236139473944e-3_f64) * t54142 + F::cast_from(0.21722835846488666732e-1_f64) * t54144 * t1047 - t54148;
    t54149
}
