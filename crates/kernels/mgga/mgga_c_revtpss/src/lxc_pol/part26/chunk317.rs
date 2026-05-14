//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 317/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk317<F: Float>(t1187: F, t1188: F, t1128: F, t1153: F, t1156: F, t1161: F, t1170: F, t1176: F, t1180: F, t300: F, t435: F, t439: F) -> (F, F, F, F) {
    let t1189 = t1187 * t1188;
    let t1193 = t300 * (-0.310907e-1 * t1156 * t435 + 1.0 * t1161 * t1170 + t1128 - t1153 - 0.19751673498613801407e-1 * t1176 + 0.5848223622634646207e0 * t1180 * t1189);
    let t1195 = 0.19751673498613801407e-1 * t300 * t1176;
    let t1196 = t300 * t439;
    (t1189, t1193, t1195, t1196)
}
