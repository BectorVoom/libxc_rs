//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 290/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk290<F: Float>(t134: F, t457: F, t461: F, t221: F, t456: F, t51: F, t972: F) -> (F, F, F, F) {
    let t1169 = t134 * t457;
    let t1170 = t1169 * t461;
    let t1171 = t221 * t1170;
    let t1173 = F::cast_from(0.27777777777777777777e-3_f64) * t456 * t1171;
    let t1174 = t51 * t972;
    (t1169, t1171, t1173, t1174)
}
