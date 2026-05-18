//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 968/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk968<F: Float>(t12052: F, t24078: F, t1089: F, t23837: F, t1024: F, t1087: F, t12047: F, t12078: F, t12149: F, t12167: F, t15670: F, t16509: F, t1685: F, t19463: F, t24075: F, t24079: F, t24084: F, t24090: F, t24093: F, t24098: F, t24104: F, t24108: F, t24112: F, t24116: F, t3204: F, t3299: F, t4857: F, t4954: F, t4981: F, t4996: F, t6362: F, t6371: F, t6375: F, t6379: F, t6383: F) -> F {
    let t24123 = t24078 * t12052;
    let t24126 = t23837 * t1089;
    let t24129 = -F::new(0.19756347548806534796e1) * t4857 * t6371 + F::new(0.39512695097613069591e1) * t3204 * t24075 - F::new(0.39512695097613069591e1) * t12078 * t24079 - F::new(0.19756347548806534796e1) * t4996 * t24084 + F::new(0.19756347548806534796e1) * t4954 * t6383 + F::new(0.39512695097613069591e1) * t4981 * t24090 + F::new(0.39512695097613069591e1) * t12167 * t24093 - F::new(0.19756347548806534796e1) * t19463 * t1685 - F::new(0.19756347548806534796e1) * t1024 * t24098 + F::new(0.39512695097613069591e1) * t15670 * t6362 + F::new(0.19756347548806534796e1) * t1087 * t24104 + F::new(0.65854491829355115987e0) * t1087 * t24108 + F::new(0.39512695097613069591e1) * t3299 * t24112 + F::new(0.19756347548806534796e1) * t1087 * t24116 + F::new(0.39512695097613069591e1) * t16509 * t6375 + F::new(0.39512695097613069591e1) * t4954 * t6379 + F::new(0.65854491829355115987e0) * t12047 * t24123 + F::new(0.39512695097613069591e1) * t12149 * t24126;
    t24129
}
