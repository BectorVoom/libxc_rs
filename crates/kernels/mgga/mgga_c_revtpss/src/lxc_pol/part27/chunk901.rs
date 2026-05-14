//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 901/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk901<F: Float>(t12048: F, t12079: F, t1071: F, t3151: F, t3304: F, t3318: F, t11687: F, t4998: F, t1086: F, t3043: F, t3075: F, t3291: F, t1082: F, t11202: F, t1024: F, t1087: F, t1090: F, t1093: F, t11788: F, t11902: F, t11940: F, t12047: F, t12053: F, t12057: F, t12066: F, t12070: F, t12074: F, t12078: F, t3278: F, t3283: F, t3299: F, t3309: F, t3313: F, t3317: F, t3322: F, t342: F, t381: F, t4996: F, t989: F) -> (F,) {
    let t12080 = t12048 * t12079;
    let t12085 = t1071 * t3151;
    let t12086 = t12085 * t3304;
    let t12089 = t12085 * t3318;
    let t12094 = t11687 * t4998;
    let t12097 = t3043 * t1086;
    let t12100 = t3291 * t3075;
    let t12105 = t1082 * t11202;
    let t12108 = 0.65854491829355115987e0 * t12047 * t12053 + 0.19756347548806534796e1 * t1087 * t12057 + 0.19756347548806534796e1 * t3043 * t1093 + 0.65854491829355115987e0 * t11902 * t381 + 0.19756347548806534796e1 * t989 * t3322 + 0.65854491829355115987e0 * t342 * t12066 + 0.65854491829355115987e0 * t1087 * t12070 - 0.19756347548806534796e1 * t1024 * t12074 - 0.39512695097613069591e1 * t12078 * t12080 + 0.39512695097613069591e1 * t3278 * t3309 + 0.39512695097613069591e1 * t3299 * t12086 - 0.19756347548806534796e1 * t3317 * t12089 + 0.19756347548806534796e1 * t3278 * t3313 - 0.19756347548806534796e1 * t4996 * t12094 + 0.19756347548806534796e1 * t12097 * t1090 - 0.19756347548806534796e1 * t1024 * t12100 + 0.39512695097613069591e1 * t11788 * t3283 - 0.39512695097613069591e1 * t11940 * t12105;
    (t12108,)
}
