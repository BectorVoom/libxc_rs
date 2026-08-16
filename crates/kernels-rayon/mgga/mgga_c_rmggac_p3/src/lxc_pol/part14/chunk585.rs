//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 585/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk585(t344: f64, t830: f64, t3839: f64, t7634: f64, t7625: f64, t7628: f64, t7629: f64, t7631: f64, t7636: f64, t7640: f64, t7643: f64, t7647: f64, t7649: f64, t7652: f64, t7654: f64, t7656: f64, t7658: f64, t7660: f64) -> (f64, f64) {
    let t7662 = t344 * t830;
    let t7663 = 0.64905642291407286545e-3_f64 * t7662;
    let t7664 = t3839 * t7634;
    let t7666 = -0.21241846568096930142e-2_f64 * t7625 - t7628 + 0.34093327067806677162e-2_f64 * t7629 - 0.45457769423742236216e-2_f64 * t7631 + 0.68186654135613354324e-2_f64 * t7636 - t7640 + 0.22728884711871118108e-1_f64 * t7643 + t7647 + 0.45360193192290319575e-3_f64 * t7649 - t7652 + 0.84672360625608596544e-3_f64 * t7654 + 0.56448240417072397695e-3_f64 * t7656 + 0.5987120850931904282e-1_f64 * t7658 - 0.99785347515531738034e-2_f64 * t7660 - t7663 - 0.13276154105060581339e-2_f64 * t7664;
    (t7663, t7666)
}
