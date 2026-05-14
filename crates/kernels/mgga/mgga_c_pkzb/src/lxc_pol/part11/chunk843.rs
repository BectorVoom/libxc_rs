//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 843/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk843<F: Float>(t5931: F, t9660: F, t2923: F, t7832: F, t3669: F, t751: F, t1138: F, t2131: F, t290: F, t2969: F, t2971: F, t2977: F, t2980: F, t2981: F, t2984: F, t3680: F, t3686: F, t3689: F, t6017: F, t6036: F, t7874: F, t791: F, t794: F, t9633: F, t9661: F, t9662: F, t9667: F, t9670: F, t9671: F, t9675: F, t9682: F, t9686: F, t9692: F, t9695: F, t9700: F) -> (F, F, F, F) {
    let t9703 = t5931 * t9660;
    let t9704 = t7832 * t2923;
    let t9707 = t751 * t3669;
    let t9712 = 0.39512695097613069591e1 * t9661 * t9662 + 0.13170898365871023197e1 * t6017 * t3680 + 0.26341796731742046394e1 * t2969 * t9667 - 0.39512695097613069591e1 * t9670 * t9671 + 0.26341796731742046394e1 * t9675 * t2971 + 0.13170898365871023197e1 * t7874 * t1138 + 0.13170898365871023197e1 * t2984 * t2977 - 0.13170898365871023197e1 * t9682 * t2981 + 0.13170898365871023197e1 * t9686 * t2971 + 0.65854491829355115987e0 * t2131 * t3686 + 0.65854491829355115987e0 * t791 * t9692 - 0.65854491829355115987e0 * t9695 * t2981 - 0.65854491829355115987e0 * t6036 * t3689 - 0.13170898365871023197e1 * t2980 * t9700 + 0.65854491829355115987e0 * t9703 * t9704 + 0.65854491829355115987e0 * t9707 * t794 + 0.65854491829355115987e0 * t290 * t9633;
    (t9703, t9704, t9707, t9712)
}
