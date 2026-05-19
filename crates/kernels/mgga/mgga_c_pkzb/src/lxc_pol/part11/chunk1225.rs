//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1225/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1225<F: Float>(t10771: F, t237: F, t732: F, t3604: F, t721: F, t1108: F, t20671: F, t7411: F, t9232: F, t20896: F, t9236: F, t29753: F, t30193: F, t30195: F, t30197: F, t30200: F, t30203: F, t30205: F, t30208: F, t30211: F, t30213: F, t30216: F, t30219: F, t30221: F, t30223: F, t30225: F, t30227: F) -> (F, F, F, F, F, F) {
    let t30228 = t237 * t10771;
    let t30230 = F::cast_from(0.5848223622634646207e0_f64) * t30228 * t732;
    let t30231 = t3604 * t721;
    let t30234 = F::cast_from(0.10526802520742363173e2_f64) * t20671 * t1108 * t30231;
    let t30236 = F::cast_from(0.96491876992155210402e2_f64) * t7411 * t9232;
    let t30238 = F::cast_from(0.1551780387578202009e4_f64) * t20896 * t9236;
    let t30239 = -t29753 - t30193 + t30195 - t30197 + t30200 + t30203 - t30205 - t30208 - t30211 + t30213 + t30216 + t30219 - t30221 + t30223 - t30225 + t30227 - t30230 - t30234 + t30236 + t30238;
    (t30230, t30231, t30234, t30236, t30238, t30239)
}
