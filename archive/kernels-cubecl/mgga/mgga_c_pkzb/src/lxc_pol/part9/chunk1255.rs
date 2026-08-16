//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1255/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1255<F: Float>(t306: F, t7575: F, t17938: F, t18290: F, t1123: F, t2111: F, t2916: F, t785: F, t17929: F, t17945: F, t17999: F, t18008: F, t18332: F, t2009: F, t2019: F, t2036: F, t21457: F, t21463: F, t21469: F, t21730: F, t2901: F, t2968: F, t2969: F, t2971: F, t2977: F, t2981: F, t5718: F, t5931: F, t5952: F, t6023: F, t6031: F, t7831: F, t7832: F, t7833: F, t7836: F, t7837: F, t7840: F, t7841: F, t7844: F, t7845: F, t7854: F, t7864: F, t7868: F, t9319: F) -> (F, F, F) {
    let t21999 = t306 * t7575;
    let t22007 = t17938 * t18290;
    let t22024 = t2111 * t1123;
    let t22056 = t785 * t2916;
    let t22060 = F::cast_from(0.39512695097613069591e1_f64) * t7837 * t6023 + F::cast_from(0.19756347548806534796e1_f64) * t6031 * t2977 + F::cast_from(0.39512695097613069591e1_f64) * t2019 * t21999 * t2971 - F::cast_from(0.11853808529283920877e2_f64) * t5718 * t7840 * t7845 - F::cast_from(0.65854491829355115987e0_f64) * t17945 * t2968 * t22007 * t21730 - F::cast_from(0.19756347548806534796e1_f64) * t7854 * t7864 + F::cast_from(0.15805078039045227836e2_f64) * t17999 * t2968 * t22007 * t21463 - F::cast_from(0.11853808529283920878e2_f64) * t7844 * t7832 * t2901 * t2009 + F::cast_from(0.19756347548806534796e1_f64) * t5931 * t7836 * t7868 - F::cast_from(0.19756347548806534796e1_f64) * t2036 * t22024 * t2981 + F::cast_from(0.39512695097613069591e1_f64) * t7841 * t6023 + F::cast_from(0.13170898365871023197e1_f64) * t2969 * t18332 + F::cast_from(0.11853808529283920877e2_f64) * t7831 * t7832 * t9319 * t2009 - F::cast_from(0.19756347548806534796e1_f64) * t2036 * t21999 * t2981 + F::cast_from(0.39512695097613069591e1_f64) * t2019 * t22024 * t2971 + F::cast_from(0.11853808529283920877e2_f64) * t5952 * t7840 * t7833 - F::cast_from(0.23707617058567841754e2_f64) * t18008 * t2968 * t22007 * t21469 + F::cast_from(0.92196288561097162379e1_f64) * t17929 * t2968 * t22007 * t21457 + F::cast_from(0.19756347548806534796e1_f64) * t5931 * t7840 * t7868 + F::cast_from(0.79025390195226139182e1_f64) * t2019 * t22056 * t2971;
    (t22007, t22056, t22060)
}
