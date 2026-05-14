//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1142/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1142<F: Float>(t1143: F, t3694: F, t10942: F, t306: F, t1123: F, t3669: F, t1133: F, t3638: F, t3650: F, t2036: F, t10979: F, t5955: F, t10993: F, t11076: F, t11079: F, t11092: F, t133: F, t17929: F, t17945: F, t17999: F, t18008: F, t18353: F, t2019: F, t22007: F, t22085: F, t26667: F, t26695: F, t2901: F, t2923: F, t2969: F, t2970: F, t2971: F, t29776: F, t2981: F, t29894: F, t30038: F, t30790: F, t3689: F, t5931: F, t759: F, t7831: F, t7832: F, t7841: F, t7844: F, t791: F, t793: F, t9319: F, t9661: F, t9667: F, t9675: F, t9682: F, t9700: F, t9704: F) -> (F, F, F, F, F, F) {
    let t30843 = t1143 * t3694;
    let t30868 = t306 * t10942;
    let t30885 = t3669 * t1123;
    let t30893 = t1133 * t3638;
    let t30897 = t1133 * t3650;
    let t30898 = t2036 * t30897;
    let t30910 = t306 * t10979;
    let t30916 = t5955 * t3650;
    let t30925 = 0.39512695097613069591e1 * t2969 * t2970 * t29894 + 0.65854491829355115987e0 * t791 * t30790 * t133 * t793 + 0.39512695097613069591e1 * t7841 * t11076 + 0.15805078039045227836e2 * t17999 * t30868 * t22007 * t29776 - 0.23707617058567841754e2 * t18008 * t30868 * t22007 * t9319 - 0.19756347548806534796e1 * t26667 * t11092 - 0.39512695097613069592e1 * t9682 * t9700 + 0.92196288561097162379e1 * t17929 * t30868 * t22007 * t2901 + 0.39512695097613069592e1 * t2019 * t30885 * t2971 - 0.19756347548806534796e1 * t26695 * t11092 + 0.79025390195226139182e1 * t9675 * t9667 + 0.19756347548806534796e1 * t5931 * t30893 * t9704 - 0.19756347548806534796e1 * t30898 * t2981 - 0.65854491829355115987e0 * t17945 * t30868 * t22007 * t2923 + 0.11853808529283920877e2 * t9661 * t7832 * t30038 - 0.39512695097613069591e1 * t18353 * t11079 + 0.13170898365871023197e1 * t2019 * t30910 * t2971 - 0.19756347548806534796e1 * t22085 * t3689 + 0.11853808529283920877e2 * t7831 * t7832 * t30916 * t759 - 0.11853808529283920877e2 * t7844 * t7832 * t10993 * t759;
    (t30843, t30885, t30893, t30897, t30910, t30925)
}
