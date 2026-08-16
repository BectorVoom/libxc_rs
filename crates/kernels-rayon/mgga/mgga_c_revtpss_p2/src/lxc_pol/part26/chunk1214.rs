//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1214/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1214(t25895: f64, t96239: f64, t26265: f64, t9686: f64, t2098: f64, t4075: f64, t786: f64, t9682: f64, t10147: f64, t2097: f64, t25921: f64, t25924: f64, t26079: f64, t26241: f64, t26282: f64, t4003: f64, t4131: f64, t4132: f64, t7295: f64, t7511: f64, t7522: f64, t7528: f64, t94610: f64, t94656: f64, t94683: f64, t96423: f64, t96432: f64, t96437: f64, t96443: f64, t96456: f64, t9658: f64, t9994: f64) -> f64 {
    let t96458 = t25895 * t96239;
    let t96460 = t26265 * t9686;
    let t96463 = t786 * t2098 * t4075;
    let t96464 = t96463 * t9682;
    let t96466 = -0.29272321618148349057e-1_f64 * t96423 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t7522 * t4131 + 0.26020884564615598386e1_f64 * t25921 * t26241 - 0.21684070470512998656e-1_f64 * t96432 - 0.65854491829355115987e0_f64 * t7511 * t10147 - 0.32927245914677557992e-1_f64 * t96437 + 0.13010442282307799193e1_f64 * t94610 * t7528 - 0.19756347548806534796e1_f64 * t26282 * t4132 + 0.26020884564615598386e1_f64 * t7295 * t94683 * t96443 * t9994 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t96443 * t4003 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t2097 * t9658 + 0.13709901006661042888e-1_f64 * t96456 - 0.86736281882051994623e-1_f64 * t96458 + 0.39029762157531132076e-1_f64 * t96460 + 0.58544643236296698113e-1_f64 * t96464;
    t96466
}
