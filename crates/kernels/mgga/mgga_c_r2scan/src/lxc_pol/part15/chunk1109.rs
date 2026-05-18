//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1109/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1109<F: Float>(t3270: F, t39311: F, t3269: F, t10634: F, t11629: F, t3262: F, t1563: F, t3582: F, t3275: F, t37299: F, t37390: F, t39276: F, t39278: F, t39282: F, t39284: F, t39289: F, t39290: F, t39295: F, t39298: F, t39303: F, t39306: F, t39309: F) -> (F, F, F, F) {
    let t39312 = t3270 * t39311;
    let t39314 = t3269 * t39312 / F::new(2.0);
    let t39317 = F::new(15.0) / F::new(8.0) * t3262 * t11629 * t10634;
    let t39318 = t3582 * t1563;
    let t39321 = F::new(585.0) / F::new(256.0) * t3275 * t37299 * t39318;
    let t39322 = -t39276 + t39278 - t39282 - t39284 + F::new(0.19211284388664477842e-2) * t37390 - t39289 + F::new(0.30487649791575028314e-3) * t39290 - t39295 - t39298 - t39303 - t39306 + F::new(0.15243824895787514157e-3) * t39309 + t39314 + t39317 + t39321;
    (t39314, t39317, t39321, t39322)
}
