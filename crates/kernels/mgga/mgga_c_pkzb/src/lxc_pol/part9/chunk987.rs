//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 987/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk987<F: Float>(t2956: F, t751: F, t1133: F, t2036: F, t1138: F, t2128: F, t2131: F, t2135: F, t2140: F, t290: F, t2969: F, t2971: F, t2977: F, t2980: F, t2981: F, t2984: F, t6023: F, t6031: F, t7804: F, t7831: F, t7833: F, t7837: F, t7841: F, t7844: F, t7845: F, t7854: F, t7858: F, t7861: F, t7864: F, t7867: F, t7868: F, t7871: F, t791: F, t794: F) -> (F, F) {
    let t7874 = t751 * t2956;
    let t7879 = t2036 * t1133;
    let t7884 = F::new(0.39512695097613069591e1) * t7831 * t7833 + F::new(0.26341796731742046394e1) * t7837 * t2971 + F::new(0.26341796731742046394e1) * t7841 * t2971 - F::new(0.39512695097613069591e1) * t7844 * t7845 + F::new(0.13170898365871023197e1) * t2969 * t6023 + F::new(0.65854491829355115987e0) * t6031 * t1138 + F::new(0.13170898365871023197e1) * t2131 * t2977 - F::new(0.13170898365871023197e1) * t7854 * t2981 + F::new(0.65854491829355115987e0) * t791 * t7858 - F::new(0.13170898365871023197e1) * t7861 * t2981 - F::new(0.65854491829355115987e0) * t2980 * t7864 + F::new(0.65854491829355115987e0) * t7867 * t7868 + F::new(0.13170898365871023197e1) * t7871 * t2128 + F::new(0.13170898365871023197e1) * t7874 * t794 + F::new(0.65854491829355115987e0) * t2984 * t2135 - F::new(0.65854491829355115987e0) * t7879 * t2140 + F::new(0.65854491829355115987e0) * t290 * t7804;
    (t7874, t7884)
}
