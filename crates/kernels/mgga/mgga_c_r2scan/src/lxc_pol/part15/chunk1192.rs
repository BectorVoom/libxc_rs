//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1192/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1192<F: Float>(t11588: F, t38355: F, t11592: F, t37400: F, t10680: F, t11587: F, t37421: F, t2768: F, t874: F, t10682: F, t10978: F, t10980: F, t11568: F) -> (F, F, F, F, F) {
    let t40303 = t38355 * t11588;
    let t40305 = t37400 * t11592;
    let t40308 = t10680 * t11587 * t37421;
    let t40310 = t2768 * t874;
    let t40312 = t10680 * t10682 * t40310;
    let t40313 = F::cast_from(0.72042316457491791906e-3_f64) * t40312;
    let t40315 = t10978 * t10980 * t11568;
    (t40303, t40305, t40308, t40313, t40315)
}
