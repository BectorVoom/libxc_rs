//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3662/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3662<F: Float>(t1168: F, t1187: F, t12476: F, t16948: F, t16959: F, t16979: F, t16989: F, t435: F, t57944: F, t57972: F, t58237: F, t58259: F, t58300: F, t58345: F, t58592: F, t58647: F, t6535: F, t68250: F, t68633: F, t68636: F, t68640: F, t68694: F, t68711: F, t68730: F, t68751: F, t68754: F, t68757: F, t69139: F, t69153: F, t69167: F, t69181: F) -> F {
    let t69192 = F::cast_from(0.14035736694323150897e2_f64) * t58345 * t16979 - F::cast_from(0.77193501593724168323e3_f64) * t57944 * t16959 + F::cast_from(0.41016075432865626631e4_f64) * t58300 * t58647 * t1187 + t68250 + F::cast_from(0.8276162067083744048e4_f64) * t58592 * t57972 * t1168 - t68633 - t68636 - t68640 + t68694 - t68711 - F::new(0.310907e-1) * (t69139 + t69153 + t69167 + t69181) * t435 - F::cast_from(0.4155806185363551302e3_f64) * t58259 * t16989 - t68730 + F::new(24.0) * t58237 * t16948 + F::cast_from(0.5848223622634646207e0_f64) * t12476 * t6535 - t68751 - t68754 + t68757;
    t69192
}
