//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 801/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk801<F: Float>(t133: F, t3016: F, t1605: F, t1604: F, t2122: F, t2139: F, t6164: F, t6196: F, t6215: F, t7553: F, t7557: F, t7582: F, t7598: F, t7603: F, t9240: F, t9244: F, t9248: F, t9251: F, t9254: F, t9258: F, t9262: F, t9270: F) -> (F, F, F) {
    let t9272 = t133 * t3016;
    let t9273 = t1605 * t9272;
    let t9274 = t1604 * t9273;
    let t9276 = -0.11557628986739024751e0 * t9240 - t6164 + 0.11643651550782197811e-1 * t9244 + 0.34930954652346593435e-1 * t9248 + 0.19514881078765566037e-1 * t9251 + 0.10975748638225852664e0 * t2122 * t9254 + 0.10975748638225852664e0 * t2122 * t9258 + 0.2600466522016280569e0 * t2139 * t9262 - t7553 - t7557 - 0.14457274399185490173e-3 * t6196 - 0.63479958930231934629e-2 * t6215 + t7582 - 0.84755945902752848174e0 * t7598 + 0.58218257753910989057e-2 * t9270 + 0.54878743191129263322e-2 * t9274 - t7603;
    (t9272, t9273, t9276)
}
