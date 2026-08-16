//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2027/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2027<F: Float>(t24649: F, t24658: F, t2131: F, t82985: F, t11713: F, t11717: F, t24727: F, t11708: F, t24732: F, t7337: F, t11835: F, t7310: F) -> (F, F, F, F, F, F) {
    let t86149 = t24658 * t24649;
    let t86154 = t2131 * t82985;
    let t86164 = t11713 * t24727 * t11717;
    let t86167 = t11708 * t24732;
    let t86171 = t11713 * t7337 * t11717;
    let t86184 = t7310 * t11835;
    (t86149, t86154, t86164, t86167, t86171, t86184)
}
