//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 987/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk987(t2956: f64, t751: f64, t1133: f64, t2036: f64, t1138: f64, t2128: f64, t2131: f64, t2135: f64, t2140: f64, t290: f64, t2969: f64, t2971: f64, t2977: f64, t2980: f64, t2981: f64, t2984: f64, t6023: f64, t6031: f64, t7804: f64, t7831: f64, t7833: f64, t7837: f64, t7841: f64, t7844: f64, t7845: f64, t7854: f64, t7858: f64, t7861: f64, t7864: f64, t7867: f64, t7868: f64, t7871: f64, t791: f64, t794: f64) -> (f64, f64) {
    let t7874 = t751 * t2956;
    let t7879 = t2036 * t1133;
    let t7884 = 0.39512695097613069591e1_f64 * t7831 * t7833 + 0.26341796731742046394e1_f64 * t7837 * t2971 + 0.26341796731742046394e1_f64 * t7841 * t2971 - 0.39512695097613069591e1_f64 * t7844 * t7845 + 0.13170898365871023197e1_f64 * t2969 * t6023 + 0.65854491829355115987e0_f64 * t6031 * t1138 + 0.13170898365871023197e1_f64 * t2131 * t2977 - 0.13170898365871023197e1_f64 * t7854 * t2981 + 0.65854491829355115987e0_f64 * t791 * t7858 - 0.13170898365871023197e1_f64 * t7861 * t2981 - 0.65854491829355115987e0_f64 * t2980 * t7864 + 0.65854491829355115987e0_f64 * t7867 * t7868 + 0.13170898365871023197e1_f64 * t7871 * t2128 + 0.13170898365871023197e1_f64 * t7874 * t794 + 0.65854491829355115987e0_f64 * t2984 * t2135 - 0.65854491829355115987e0_f64 * t7879 * t2140 + 0.65854491829355115987e0_f64 * t290 * t7804;
    (t7874, t7884)
}
