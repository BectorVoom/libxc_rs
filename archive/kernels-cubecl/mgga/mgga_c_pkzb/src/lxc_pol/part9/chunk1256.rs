//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1256/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1256<F: Float>(t751: F, t7804: F, t1133: F, t5718: F, t2019: F, t2956: F, t2036: F, t5931: F, t5952: F, t1138: F, t133: F, t18284: F, t2009: F, t2128: F, t2131: F, t2135: F, t2140: F, t21822: F, t21843: F, t21912: F, t21964: F, t22056: F, t287: F, t290: F, t2970: F, t2980: F, t2981: F, t2984: F, t6014: F, t6023: F, t6028: F, t6040: F, t6049: F, t759: F, t7832: F, t7833: F, t7836: F, t7845: F, t7858: F, t7861: F, t7864: F, t7867: F, t7874: F, t791: F, t793: F, t794: F) -> F {
    let t22063 = t751 * t7804;
    let t22074 = t5718 * t1133;
    let t22082 = t2019 * t2956;
    let t22085 = t2036 * t2956;
    let t22111 = t5931 * t1133;
    let t22114 = t5952 * t1133;
    let t22119 = F::cast_from(0.19756347548806534796e1_f64) * t7874 * t2135 + F::cast_from(0.19756347548806534796e1_f64) * t22063 * t794 - F::cast_from(0.19756347548806534796e1_f64) * t2036 * t1133 * t2009 * t2981 + F::cast_from(0.65854491829355115987e0_f64) * t18284 * t1138 - F::cast_from(0.19756347548806534796e1_f64) * t7861 * t7864 - F::cast_from(0.39512695097613069591e1_f64) * t22074 * t6028 - F::cast_from(0.65854491829355115987e0_f64) * t2980 * t2970 * t21843 + F::cast_from(0.65854491829355115987e0_f64) * t290 * t21964 + F::cast_from(0.39512695097613069591e1_f64) * t22082 * t2128 - F::cast_from(0.19756347548806534796e1_f64) * t22085 * t2140 + F::cast_from(0.65854491829355115987e0_f64) * t2984 * t6040 + F::cast_from(0.11853808529283920877e2_f64) * t5952 * t7836 * t7833 + F::cast_from(0.39512695097613069591e1_f64) * t2019 * t1133 * t759 * t6023 - F::cast_from(0.11853808529283920877e2_f64) * t5718 * t7836 * t7845 - F::cast_from(0.39512695097613069591e1_f64) * t2036 * t22056 * t2981 + F::cast_from(0.19756347548806534796e1_f64) * t7867 * t7832 * t21912 * t287 + F::cast_from(0.65854491829355115987e0_f64) * t791 * t21822 * t133 * t793 + F::cast_from(0.65854491829355115987e0_f64) * t22111 * t6049 + F::cast_from(0.39512695097613069591e1_f64) * t22114 * t6014 + F::cast_from(0.19756347548806534796e1_f64) * t2131 * t7858;
    t22119
}
