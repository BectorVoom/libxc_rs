//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3413/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3413<F: Float>(t4669: F, t19045: F, t964: F, t3011: F, t6184: F, t11450: F, t11456: F, t11548: F, t15235: F, t15249: F, t15259: F, t15274: F, t15339: F, t15343: F, t1622: F, t1634: F, t19156: F, t19266: F, t2943: F, t2944: F, t2987: F, t3007: F, t3015: F, t41785: F, t4685: F, t4708: F, t52264: F, t52320: F, t52430: F, t52511: F, t52522: F, t52840: F, t6177: F, t6190: F, t6206: F, t63612: F, t63902: F, t953: F, t954: F, t973: F, t974: F) -> (F, F) {
    let t64109 = t4669 * t4669;
    let t64120 = t19045 * t964;
    let t64125 = t6184 * t3011;
    let t64146 = F::cast_from(0.11579025239058625248e4_f64) * t11450 * t6177 * t2944 - F::cast_from(8.0_f64) * t11548 * t19266 - F::cast_from(4.0_f64) * t2943 * t64109 * t954 - F::cast_from(0.23392894490538584828e1_f64) * t2987 * t63902 * t973 + F::cast_from(0.14035736694323150897e2_f64) * t52430 * t15249 + F::cast_from(24.0_f64) * t52840 * t15274 - t63612 + F::cast_from(0.11696447245269292414e1_f64) * t64120 * t974 + F::cast_from(0.5848223622634646207e0_f64) * t19156 * t3007 + F::cast_from(0.17315859105681463759e2_f64) * t64125 * t3015 + F::cast_from(0.11696447245269292414e1_f64) * t52522 * t1634 + F::cast_from(0.23392894490538584828e1_f64) * t15343 * t4708 + F::cast_from(0.11696447245269292414e1_f64) * t4685 * t15235 - F::cast_from(0.11696447245269292414e1_f64) * t41785 * t6190 + F::cast_from(0.5848223622634646207e0_f64) * t11456 * t6206 + F::cast_from(0.8276162067083744048e4_f64) * t52320 * t52264 * t953 - F::cast_from(0.4155806185363551302e3_f64) * t52511 * t15259 - F::cast_from(4.0_f64) * t2943 * t1622 * t15339;
    (t64109, t64146)
}
