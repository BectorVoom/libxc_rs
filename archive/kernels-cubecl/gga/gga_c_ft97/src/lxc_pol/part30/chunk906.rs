//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 906/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk906<F: Float>(t1095: F, t665: F, t10696: F, t1240: F, t2842: F, t4239: F, t2770: F, t4246: F, t309: F, t798: F, t2681: F, t863: F) -> (F, F, F, F, F, F) {
    let t55109 = t665 * t1095;
    let t55768 = t1240 * t10696;
    let t55797 = t4239 * t2842;
    let t56098 = t2770 * t4246;
    let t56110 = t798 * t309;
    let t56127 = t2681 * t863;
    (t55109, t55768, t55797, t56098, t56110, t56127)
}
