//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 577/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk577<F: Float>(t4261: F, t7893: F, t9074: F, t2312: F, t3351: F, t7974: F, t894: F, t1063: F, t9097: F, t9100: F, t9108: F, t9111: F, t9113: F, t9115: F) -> (F, F, F, F) {
    let t10185 = t4261 * t7893;
    let t10186 = t9074 * t10185;
    let t10187 = F::cast_from(0.23712505529730124666e-2_f64) * t10186;
    let t10194 = t2312 * t3351;
    let t10195 = F::cast_from(0.11856252764865062333e-2_f64) * t10194;
    let t10196 = t894 * t7974;
    let t10198 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t10196;
    let t10205 = -F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t9097 + F::cast_from(147.0_f64) / F::cast_from(8192.0_f64) * t9100 - F::cast_from(63.0_f64) / F::cast_from(524288.0_f64) * t9108 + F::cast_from(21.0_f64) / F::cast_from(524288.0_f64) * t9111 - F::cast_from(49.0_f64) / F::cast_from(8192.0_f64) * t9113 + F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t9115;
    (t10187, t10195, t10198, t10205)
}
