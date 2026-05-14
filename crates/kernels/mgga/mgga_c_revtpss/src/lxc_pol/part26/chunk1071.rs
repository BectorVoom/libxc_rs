//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1071/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1071<F: Float>(t96463: F, t9682: F, t10147: F, t2097: F, t25921: F, t25924: F, t26079: F, t26241: F, t26282: F, t4003: F, t4131: F, t4132: F, t7295: F, t7511: F, t7522: F, t7528: F, t94610: F, t94656: F, t94683: F, t96423: F, t96432: F, t96437: F, t96443: F, t96456: F, t96458: F, t96460: F, t9658: F, t9994: F) -> (F,) {
    let t96464 = t96463 * t9682;
    let t96466 = -0.29272321618148349057e-1 * t96423 - 0.78062653693846795158e1 * t7295 * t25924 * t7522 * t4131 + 0.26020884564615598386e1 * t25921 * t26241 - 0.21684070470512998656e-1 * t96432 - 0.65854491829355115987e0 * t7511 * t10147 - 0.32927245914677557992e-1 * t96437 + 0.13010442282307799193e1 * t94610 * t7528 - 0.19756347548806534796e1 * t26282 * t4132 + 0.26020884564615598386e1 * t7295 * t94683 * t96443 * t9994 - 0.26020884564615598386e1 * t7295 * t26079 * t96443 * t4003 + 0.10408353825846239354e2 * t7295 * t94656 * t2097 * t9658 + 0.13709901006661042888e-1 * t96456 - 0.86736281882051994623e-1 * t96458 + 0.39029762157531132076e-1 * t96460 + 0.58544643236296698113e-1 * t96464;
    (t96466,)
}
