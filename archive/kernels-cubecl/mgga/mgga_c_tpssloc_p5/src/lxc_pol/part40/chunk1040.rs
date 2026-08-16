//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1040/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1040<F: Float>(t9869: F, t5519: F, t706: F, t708: F, t9871: F, t13115: F, t157: F, t4196: F, t9880: F, t13107: F, t13105: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> (F, F, F, F, F, F, F) {
    let t16688 = F::cast_from(4.0_f64) * t9869;
    let t16689 = t706 * t5519;
    let t16691 = F::cast_from(4.0_f64) * t16689 * t708;
    let t16692 = F::cast_from(0.24415263074675393405e-3_f64) * t9871;
    let t16693 = t13115 * t157;
    let t16695 = F::cast_from(24.0_f64) * t16693 * t4196;
    let t16696 = F::cast_from(0.10843581300301739842e-1_f64) * t9880;
    let t16697 = F::cast_from(0.34631718211362927517e2_f64) * t13107;
    let t16698 = t16688 + t16691 + t16692 + t9793 + t9797 - t9876 + t13105 - t9820 - t9824 + t16695 + t16696 - t9884 + t9887 + t9890 - t16697;
    (t16688, t16691, t16692, t16695, t16696, t16697, t16698)
}
