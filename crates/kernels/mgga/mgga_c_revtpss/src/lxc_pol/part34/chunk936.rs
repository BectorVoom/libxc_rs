//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 936/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk936<F: Float>(t24111: F, t3304: F, t1089: F, t1668: F, t6343: F, t12052: F, t24078: F, t23837: F, t1024: F, t1087: F, t12047: F, t12078: F, t12149: F, t12167: F, t15670: F, t16509: F, t1685: F, t19463: F, t24075: F, t24079: F, t24084: F, t24090: F, t24093: F, t24098: F, t24104: F, t24108: F, t3204: F, t3299: F, t4857: F, t4954: F, t4981: F, t4996: F, t6362: F, t6371: F, t6375: F, t6379: F, t6383: F) -> (F,) {
    let t24112 = t24111 * t3304;
    let t24116 = t6343 * t1668 * t1089;
    let t24123 = t24078 * t12052;
    let t24126 = t23837 * t1089;
    let t24129 = -0.19756347548806534796e1 * t4857 * t6371 + 0.39512695097613069591e1 * t3204 * t24075 - 0.39512695097613069591e1 * t12078 * t24079 - 0.19756347548806534796e1 * t4996 * t24084 + 0.19756347548806534796e1 * t4954 * t6383 + 0.39512695097613069591e1 * t4981 * t24090 + 0.39512695097613069591e1 * t12167 * t24093 - 0.19756347548806534796e1 * t19463 * t1685 - 0.19756347548806534796e1 * t1024 * t24098 + 0.39512695097613069591e1 * t15670 * t6362 + 0.19756347548806534796e1 * t1087 * t24104 + 0.65854491829355115987e0 * t1087 * t24108 + 0.39512695097613069591e1 * t3299 * t24112 + 0.19756347548806534796e1 * t1087 * t24116 + 0.39512695097613069591e1 * t16509 * t6375 + 0.39512695097613069591e1 * t4954 * t6379 + 0.65854491829355115987e0 * t12047 * t24123 + 0.39512695097613069591e1 * t12149 * t24126;
    (t24129,)
}
