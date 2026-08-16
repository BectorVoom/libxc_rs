//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 866/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk866<F: Float>(t3247: F, t405: F, t974: F, t11147: F, t461: F, t457: F, t63: F, t221: F, t456: F, t3242: F, t460: F, t1176: F, t134: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11545 = F::cast_from(1.0_f64) / t405 / t3247;
    let t11546 = t974 * t11545;
    let t11547 = t461 * t11147;
    let t11552 = t63 * t457;
    let t11553 = t11552 * t461;
    let t11554 = t221 * t11553;
    let t11556 = F::cast_from(0.3086419753086419753e-3_f64) * t456 * t11554;
    let t11570 = t460 * t3242;
    let t11583 = t460 * t3247;
    let t11588 = t134 * t1176;
    (t11545, t11546, t11547, t11552, t11553, t11556, t11570, t11583, t11588)
}
