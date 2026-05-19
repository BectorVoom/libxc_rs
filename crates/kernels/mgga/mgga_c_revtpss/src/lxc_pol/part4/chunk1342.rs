//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1342/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1342<F: Float>(t141: F, t16886: F, t1145: F, t16733: F, t5098: F, t698: F, t16725: F, t3417: F, t16729: F, t16720: F, t16738: F, t12254: F, t16715: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16887 = t141 * t16886;
    let t16889 = t1145 * t16733;
    let t16890 = t141 * t16889;
    let t16892 = t698 * t5098;
    let t16893 = F::cast_from(0.21908444444444444444e0_f64) * t16892;
    let t16894 = t3417 * t16725;
    let t16895 = t141 * t16894;
    let t16897 = t3417 * t16729;
    let t16898 = t141 * t16897;
    let t16900 = t3417 * t16720;
    let t16901 = t141 * t16900;
    let t16903 = t1145 * t16738;
    let t16904 = t141 * t16903;
    let t16907 = t12254 * t16715;
    (t16887, t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16907)
}
