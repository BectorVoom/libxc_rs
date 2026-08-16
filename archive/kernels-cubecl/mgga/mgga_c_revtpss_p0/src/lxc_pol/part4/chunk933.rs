//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 933/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk933<F: Float>(t159: F, t793: F, t587: F, t65: F, t4153: F, t575: F, t1455: F, t1464: F, t4168: F, t571: F, t143: F, t2580: F) -> (F, F, F, F, F, F) {
    let t7021 = t793 * t159;
    let t8779 = F::cast_from(1.0_f64) / t65 / t587;
    let t9263 = t4153 * t575;
    let t9265 = t1455 * t1464;
    let t9267 = t571 * t4168;
    let t9273 = F::cast_from(1.0_f64) / t2580 / t143;
    (t7021, t8779, t9263, t9265, t9267, t9273)
}
