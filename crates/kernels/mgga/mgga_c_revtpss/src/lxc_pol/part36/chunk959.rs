//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 959/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk959<F: Float>(t1089: F, t23992: F, t23997: F, t24007: F, t3304: F, t3318: F, t5004: F, t6244: F, t1082: F, t24031: F, t24111: F, t23598: F, t24042: F, t380: F, t6258: F, t1024: F, t11940: F, t12122: F, t12127: F, t1647: F, t16502: F, t16544: F, t16584: F, t1689: F, t1692: F, t19566: F, t23959: F, t3204: F, t3287: F, t3317: F, t342: F, t381: F, t4857: F, t6235: F, t6365: F, t6368: F, t6386: F, t6389: F) -> (F,) {
    let t24132 = t23992 * t1089;
    let t24135 = t23997 * t1089;
    let t24138 = t24007 * t3304;
    let t24141 = t24007 * t3318;
    let t24144 = t5004 * t6244;
    let t24147 = t1082 * t24031;
    let t24152 = t24111 * t3318;
    let t24157 = t1082 * t23598;
    let t24162 = t380 * t24042;
    let t24167 = t5004 * t6258;
    let t24176 = -0.39512695097613069591e1 * t16544 * t6365 - 0.19756347548806534796e1 * t3287 * t24132 - 0.19756347548806534796e1 * t3287 * t24135 - 0.39512695097613069591e1 * t12122 * t24138 + 0.19756347548806534796e1 * t12127 * t24141 + 0.39512695097613069591e1 * t3204 * t24144 - 0.39512695097613069591e1 * t11940 * t24147 + 0.19756347548806534796e1 * t6235 * t1692 - 0.19756347548806534796e1 * t3317 * t24152 + 0.65854491829355115987e0 * t23959 * t381 - 0.65854491829355115987e0 * t1024 * t24157 + 0.19756347548806534796e1 * t1647 * t6389 + 0.65854491829355115987e0 * t342 * t24162 - 0.39512695097613069591e1 * t16502 * t6365 - 0.19756347548806534796e1 * t1024 * t24167 - 0.39512695097613069591e1 * t4857 * t6368 + 0.19756347548806534796e1 * t19566 * t1689 - 0.19756347548806534796e1 * t16584 * t6386;
    (t24176,)
}
