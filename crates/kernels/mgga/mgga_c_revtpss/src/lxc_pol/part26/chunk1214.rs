//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1214/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1214<F: Float>(t25895: F, t96239: F, t26265: F, t9686: F, t2098: F, t4075: F, t786: F, t9682: F, t10147: F, t2097: F, t25921: F, t25924: F, t26079: F, t26241: F, t26282: F, t4003: F, t4131: F, t4132: F, t7295: F, t7511: F, t7522: F, t7528: F, t94610: F, t94656: F, t94683: F, t96423: F, t96432: F, t96437: F, t96443: F, t96456: F, t9658: F, t9994: F) -> F {
    let t96458 = t25895 * t96239;
    let t96460 = t26265 * t9686;
    let t96463 = t786 * t2098 * t4075;
    let t96464 = t96463 * t9682;
    let t96466 = -F::cast_from(0.29272321618148349057e-1_f64) * t96423 - F::cast_from(0.78062653693846795158e1_f64) * t7295 * t25924 * t7522 * t4131 + F::cast_from(0.26020884564615598386e1_f64) * t25921 * t26241 - F::cast_from(0.21684070470512998656e-1_f64) * t96432 - F::cast_from(0.65854491829355115987e0_f64) * t7511 * t10147 - F::cast_from(0.32927245914677557992e-1_f64) * t96437 + F::cast_from(0.13010442282307799193e1_f64) * t94610 * t7528 - F::cast_from(0.19756347548806534796e1_f64) * t26282 * t4132 + F::cast_from(0.26020884564615598386e1_f64) * t7295 * t94683 * t96443 * t9994 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t26079 * t96443 * t4003 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t2097 * t9658 + F::cast_from(0.13709901006661042888e-1_f64) * t96456 - F::cast_from(0.86736281882051994623e-1_f64) * t96458 + F::cast_from(0.39029762157531132076e-1_f64) * t96460 + F::cast_from(0.58544643236296698113e-1_f64) * t96464;
    t96466
}
