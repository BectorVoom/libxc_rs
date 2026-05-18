//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 285/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk285<F: Float>(t1176: F, t367: F, t374: F, t365: F, t366: F, t1072: F, t54: F, t1076: F, t1126: F, t1131: F, t1138: F, t1153: F, t1161: F, t1165: F, t1169: F) -> (F, F, F, F, F, F, F) {
    let t1178 = t367 * t1176 * t374;
    let t1180 = F::new(0.58482233974552040708e0) * t365 * t1178;
    let t1181 = t366 * t366;
    let t1182 = F::new(1.0) / t1181;
    let t1183 = t1182 * t1072;
    let t1184 = t54 * t54;
    let t1185 = F::new(1.0) / t1184;
    let t1186 = t1183 * t1185;
    let t1188 = F::new(0.17315755899375863299e2) * t365 * t1186;
    let t1189 = -t1126 - t1131 - t1138 + t1153 + t1161 + t1165 + t1169 + t1076 - t1180 - t1188;
    (t1178, t1180, t1182, t1185, t1186, t1188, t1189)
}
