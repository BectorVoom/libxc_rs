//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 892/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk892(t2923: f64, t7832: f64, t3669: f64, t751: f64, t1138: f64, t2131: f64, t290: f64, t2969: f64, t2971: f64, t2977: f64, t2980: f64, t2981: f64, t2984: f64, t3680: f64, t3686: f64, t3689: f64, t6017: f64, t6036: f64, t7874: f64, t791: f64, t794: f64, t9633: f64, t9661: f64, t9662: f64, t9667: f64, t9670: f64, t9671: f64, t9675: f64, t9682: f64, t9686: f64, t9692: f64, t9695: f64, t9700: f64, t9703: f64) -> (f64, f64, f64) {
    let t9704 = t7832 * t2923;
    let t9707 = t751 * t3669;
    let t9712 = 0.39512695097613069591e1_f64 * t9661 * t9662 + 0.13170898365871023197e1_f64 * t6017 * t3680 + 0.26341796731742046394e1_f64 * t2969 * t9667 - 0.39512695097613069591e1_f64 * t9670 * t9671 + 0.26341796731742046394e1_f64 * t9675 * t2971 + 0.13170898365871023197e1_f64 * t7874 * t1138 + 0.13170898365871023197e1_f64 * t2984 * t2977 - 0.13170898365871023197e1_f64 * t9682 * t2981 + 0.13170898365871023197e1_f64 * t9686 * t2971 + 0.65854491829355115987e0_f64 * t2131 * t3686 + 0.65854491829355115987e0_f64 * t791 * t9692 - 0.65854491829355115987e0_f64 * t9695 * t2981 - 0.65854491829355115987e0_f64 * t6036 * t3689 - 0.13170898365871023197e1_f64 * t2980 * t9700 + 0.65854491829355115987e0_f64 * t9703 * t9704 + 0.65854491829355115987e0_f64 * t9707 * t794 + 0.65854491829355115987e0_f64 * t290 * t9633;
    (t9704, t9707, t9712)
}
