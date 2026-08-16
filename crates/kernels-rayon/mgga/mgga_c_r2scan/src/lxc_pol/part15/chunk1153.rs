//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1153/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1153(t39825: f64, t24573: f64, t37717: f64, t26145: f64, t37716: f64, t571: f64, t24521: f64, t37720: f64, t11780: f64, t783: f64, t788: f64, t37920: f64, t39814: f64, t39816: f64, t39818: f64, t39821: f64, t39824: f64) -> f64 {
    let t39826 = 0.14282990759302185292e-1_f64 * t39825;
    let t39827 = t37717 * t24573;
    let t39828 = 0.47609969197673950972e-2_f64 * t39827;
    let t39830 = t571 * t37716 * t26145;
    let t39831 = 0.47609969197673950972e-2_f64 * t39830;
    let t39832 = t37720 * t24521;
    let t39835 = t783 * t11780 * t788;
    let t39836 = 0.46574606203128791246e-1_f64 * t39835;
    let t39837 = -0.10975748638225852664e0_f64 * t39814 + 0.59512461497092438715e-1_f64 * t39816 - 0.43663693315433241792e-2_f64 * t39818 + 0.86682217400542685632e-1_f64 * t39821 - t39824 - t39826 - t39828 + t39831 - 0.14282990759302185291e-1_f64 * t39832 - t37920 + t39836;
    t39837
}
