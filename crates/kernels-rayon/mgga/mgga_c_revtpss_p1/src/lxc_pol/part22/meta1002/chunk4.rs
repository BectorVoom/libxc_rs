//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3413/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3413(t4669: f64, t19045: f64, t964: f64, t3011: f64, t6184: f64, t11450: f64, t11456: f64, t11548: f64, t15235: f64, t15249: f64, t15259: f64, t15274: f64, t15339: f64, t15343: f64, t1622: f64, t1634: f64, t19156: f64, t19266: f64, t2943: f64, t2944: f64, t2987: f64, t3007: f64, t3015: f64, t41785: f64, t4685: f64, t4708: f64, t52264: f64, t52320: f64, t52430: f64, t52511: f64, t52522: f64, t52840: f64, t6177: f64, t6190: f64, t6206: f64, t63612: f64, t63902: f64, t953: f64, t954: f64, t973: f64, t974: f64) -> (f64, f64) {
    let t64109 = t4669 * t4669;
    let t64120 = t19045 * t964;
    let t64125 = t6184 * t3011;
    let t64146 = 0.11579025239058625248e4_f64 * t11450 * t6177 * t2944 - 8.0_f64 * t11548 * t19266 - 4.0_f64 * t2943 * t64109 * t954 - 0.23392894490538584828e1_f64 * t2987 * t63902 * t973 + 0.14035736694323150897e2_f64 * t52430 * t15249 + 24.0_f64 * t52840 * t15274 - t63612 + 0.11696447245269292414e1_f64 * t64120 * t974 + 0.5848223622634646207e0_f64 * t19156 * t3007 + 0.17315859105681463759e2_f64 * t64125 * t3015 + 0.11696447245269292414e1_f64 * t52522 * t1634 + 0.23392894490538584828e1_f64 * t15343 * t4708 + 0.11696447245269292414e1_f64 * t4685 * t15235 - 0.11696447245269292414e1_f64 * t41785 * t6190 + 0.5848223622634646207e0_f64 * t11456 * t6206 + 0.8276162067083744048e4_f64 * t52320 * t52264 * t953 - 0.4155806185363551302e3_f64 * t52511 * t15259 - 4.0_f64 * t2943 * t1622 * t15339;
    (t64109, t64146)
}
