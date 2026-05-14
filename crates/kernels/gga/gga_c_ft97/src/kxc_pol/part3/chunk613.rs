//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 613/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk613<F: Float>(t2842: F, t863: F, t869: F, t309: F, t2770: F, t871: F, t8232: F, t837: F, t877: F, t313: F, t89: F, t9555: F, t295: F, t9568: F, t842: F, t10397: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10688 = t863 * t2842;
    let t10695 = t869 * t869;
    let t10696 = 1.0 / t10695;
    let t10697 = t309 * t10696;
    let t10703 = t2770 * t871;
    let t10732 = t8232 * t837;
    let t10735 = t8232 * t877;
    let t10749 = 28.0 / 81.0 * t89 * t9555 * t313;
    let t10758 = t9568 * t295;
    let t10773 = t8232 * t842;
    let t10797 = 28.0 / 27.0 * t10397;
    (t10688, t10697, t10703, t10732, t10735, t10749, t10758, t10773, t10797)
}
