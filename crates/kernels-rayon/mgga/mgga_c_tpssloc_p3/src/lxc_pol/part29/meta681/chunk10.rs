//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2303/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2303(t27533: f64, t86094: f64, t24826: f64, t27521: f64, t1235: f64, t1244: f64, t1246: f64, t1734: f64, t24589: f64, t24745: f64, t24757: f64, t24777: f64, t24788: f64, t24858: f64, t27453: f64, t27454: f64, t27465: f64, t27516: f64, t27549: f64, t27550: f64, t3242: f64, t3961: f64, t7283: f64, t8066: f64, t85832: f64, t86001: f64, t94400: f64, t94404: f64) -> f64 {
    let t95163 = 0.18277045187202515961e-2_f64 * t86094 * t27533;
    let t95165 = 0.54831135561607547884e-2_f64 * t24826 * t27521;
    let t95184 = -0.16449340668482264365e-1_f64 * t7283 * t94400 * t27454 - 0.82246703342411321825e-2_f64 * t7283 * t94404 * t27454 - 0.16449340668482264365e-1_f64 * t7283 * t27453 * t24745 * t24858 - t95163 + t95165 + 0.73108180748810063846e-2_f64 * t27549 * t27550 * t1235 * t3242 * t3961 + 0.27415567780803773942e-2_f64 * t24589 * t85832 * t8066 + 0.54831135561607547884e-2_f64 * t24589 * t24788 * t27465 - 0.36554090374405031923e-2_f64 * t27549 * t27516 * t24777 + t1244 * t24757 * t1734 * t1246 - 0.54831135561607547884e-2_f64 * t86001;
    t95184
}
