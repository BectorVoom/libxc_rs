//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3621/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3621<F: Float>(t1187: F, t5180: F, t16997: F, t58672: F, t20567: F, t300: F, t1198: F, t20400: F, t3539: F, t5501: F, t5184: F, t58665: F) -> (F, F, F, F, F, F) {
    let t68605 = t1187 * t5180;
    let t68608 = F::cast_from(0.41016075432865626631e4_f64) * t58672 * t16997 * t68605;
    let t68609 = t300 * t20567;
    let t68611 = F::cast_from(0.11696447245269292414e1_f64) * t68609 * t1198;
    let t68613 = F::cast_from(0.5848223622634646207e0_f64) * t20400 * t3539;
    let t68614 = t5501 * t5501;
    let t68621 = F::cast_from(0.4155806185363551302e3_f64) * t58665 * t5184 * t68605;
    (t68605, t68608, t68611, t68613, t68614, t68621)
}
