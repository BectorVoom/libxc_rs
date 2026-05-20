//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2923/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2923<F: Float>(t11213: F, t1678: F, t3059: F, t4772: F, t16237: F, t342: F, t1000: F, t1073: F, t1076: F, t1079: F, t1097: F, t11121: F, t11122: F, t11177: F, t11184: F, t11195: F, t11201: F, t11214: F, t11220: F, t12043: F, t12173: F, t12178: F, t15886: F, t16371: F, t1651: F, t1652: F, t16600: F, t16603: F, t1695: F, t1696: F, t19428: F, t3075: F, t3269: F, t3326: F, t42107: F, t43687: F, t43696: F, t43707: F, t4758: F, t4764: F, t4778: F, t5015: F, t5016: F, t995: F, t996: F) -> (F, F) {
    let t53058 = t11213 * t1678;
    let t53089 = t4772 * t3059;
    let t53093 = t342 * t16237;
    let t53107 = -F::cast_from(0.19756347548806534796e1_f64) * t53058 * t1000 + F::cast_from(0.19756347548806534796e1_f64) * t15886 * t1073 + F::cast_from(0.19756347548806534796e1_f64) * t4778 * t11184 - F::cast_from(0.19756347548806534796e1_f64) * t43696 * t1652 + F::cast_from(0.39512695097613069591e1_f64) * t995 * t11121 * t1651 * t11122 + F::cast_from(0.39512695097613069591e1_f64) * t16600 * t12043 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t5015 * t3075 - F::cast_from(0.39512695097613069591e1_f64) * t11220 * t5016 + F::cast_from(0.39512695097613069591e1_f64) * t42107 * t4758 + F::cast_from(0.13170898365871023197e1_f64) * t1076 * t3269 * t1695 * t12173 - F::cast_from(0.19756347548806534796e1_f64) * t16371 * t3326 + F::cast_from(0.19756347548806534796e1_f64) * t11214 * t4764 - F::cast_from(0.11853808529283920877e2_f64) * t11201 * t996 * t53089 - F::cast_from(0.19756347548806534796e1_f64) * t53093 * t1097 - F::cast_from(0.39512695097613069591e1_f64) * t16603 * t19428 * t11177 - F::cast_from(0.19756347548806534796e1_f64) * t43687 * t1696 - F::cast_from(0.19756347548806534796e1_f64) * t11195 * t5016 - F::cast_from(0.39512695097613069591e1_f64) * t16600 * t12178 - F::cast_from(0.65854491829355115987e0_f64) * t43707 * t1652;
    (t53089, t53107)
}
