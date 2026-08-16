//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 872/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk872(t3177: f64, t784: f64, t783: f64, t788: f64, t133: f64, t3016: f64, t1605: f64, t1604: f64, t2122: f64, t2139: f64, t6164: f64, t6196: f64, t6215: f64, t7553: f64, t7557: f64, t7582: f64, t7598: f64, t7603: f64, t9240: f64, t9244: f64, t9248: f64, t9251: f64, t9254: f64, t9258: f64, t9262: f64) -> (f64, f64, f64) {
    let t9268 = t3177 * t784;
    let t9270 = t783 * t9268 * t788;
    let t9272 = t133 * t3016;
    let t9273 = t1605 * t9272;
    let t9274 = t1604 * t9273;
    let t9276 = -0.11557628986739024751e0_f64 * t9240 - t6164 + 0.11643651550782197811e-1_f64 * t9244 + 0.34930954652346593435e-1_f64 * t9248 + 0.19514881078765566037e-1_f64 * t9251 + 0.10975748638225852664e0_f64 * t2122 * t9254 + 0.10975748638225852664e0_f64 * t2122 * t9258 + 0.2600466522016280569e0_f64 * t2139 * t9262 - t7553 - t7557 - 0.14457274399185490173e-3_f64 * t6196 - 0.63479958930231934629e-2_f64 * t6215 + t7582 - 0.84755945902752848174e0_f64 * t7598 + 0.58218257753910989057e-2_f64 * t9270 + 0.54878743191129263322e-2_f64 * t9274 - t7603;
    (t9272, t9273, t9276)
}
