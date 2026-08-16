//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1261/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1261<F: Float>(t10993: F, t11076: F, t11079: F, t11092: F, t133: F, t17929: F, t17945: F, t17999: F, t18008: F, t18353: F, t2019: F, t22007: F, t22085: F, t26667: F, t26695: F, t2901: F, t2923: F, t2969: F, t2970: F, t2971: F, t29776: F, t2981: F, t29894: F, t30038: F, t30790: F, t30868: F, t30885: F, t30893: F, t30898: F, t30910: F, t30916: F, t3689: F, t5931: F, t759: F, t7831: F, t7832: F, t7841: F, t7844: F, t791: F, t793: F, t9319: F, t9661: F, t9667: F, t9675: F, t9682: F, t9700: F, t9704: F) -> F {
    let t30925 = F::cast_from(0.39512695097613069591e1_f64) * t2969 * t2970 * t29894 + F::cast_from(0.65854491829355115987e0_f64) * t791 * t30790 * t133 * t793 + F::cast_from(0.39512695097613069591e1_f64) * t7841 * t11076 + F::cast_from(0.15805078039045227836e2_f64) * t17999 * t30868 * t22007 * t29776 - F::cast_from(0.23707617058567841754e2_f64) * t18008 * t30868 * t22007 * t9319 - F::cast_from(0.19756347548806534796e1_f64) * t26667 * t11092 - F::cast_from(0.39512695097613069592e1_f64) * t9682 * t9700 + F::cast_from(0.92196288561097162379e1_f64) * t17929 * t30868 * t22007 * t2901 + F::cast_from(0.39512695097613069592e1_f64) * t2019 * t30885 * t2971 - F::cast_from(0.19756347548806534796e1_f64) * t26695 * t11092 + F::cast_from(0.79025390195226139182e1_f64) * t9675 * t9667 + F::cast_from(0.19756347548806534796e1_f64) * t5931 * t30893 * t9704 - F::cast_from(0.19756347548806534796e1_f64) * t30898 * t2981 - F::cast_from(0.65854491829355115987e0_f64) * t17945 * t30868 * t22007 * t2923 + F::cast_from(0.11853808529283920877e2_f64) * t9661 * t7832 * t30038 - F::cast_from(0.39512695097613069591e1_f64) * t18353 * t11079 + F::cast_from(0.13170898365871023197e1_f64) * t2019 * t30910 * t2971 - F::cast_from(0.19756347548806534796e1_f64) * t22085 * t3689 + F::cast_from(0.11853808529283920877e2_f64) * t7831 * t7832 * t30916 * t759 - F::cast_from(0.11853808529283920877e2_f64) * t7844 * t7832 * t10993 * t759;
    t30925
}
