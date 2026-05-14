//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1277/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1277<F: Float>(t1123: F, t2956: F, t306: F, t9539: F, t3638: F, t785: F, t2916: F, t2029: F, t18304: F, t18326: F, t2019: F, t2036: F, t2131: F, t2138: F, t2139: F, t2923: F, t2971: F, t2981: F, t3680: F, t3689: F, t5718: F, t6023: F, t7658: F, t7832: F, t7854: F, t7867: F, t9671: F, t9686: F, t9692: F, t9700: F, t9703: F) -> (F, F, F, F, F) {
    let t25095 = t2956 * t1123;
    let t25113 = t306 * t9539;
    let t25117 = t785 * t3638;
    let t25121 = t2916 * t2916;
    let t25122 = t25121 * t2029;
    let t25126 = 0.26341796731742046394e1 * t7867 * t7832 * t2923 * t2916 - 0.26341796731742046394e1 * t7854 * t9700 + 0.52683593463484092788e1 * t2019 * t25095 * t2971 + 0.13170898365871023197e1 * t9686 * t6023 + 0.65854491829355115987e0 * t9703 * t7832 * t7658 + 0.13170898365871023197e1 * t18304 * t3680 - 0.26341796731742046394e1 * t2036 * t25095 * t2981 + 0.13170898365871023197e1 * t2131 * t9692 - 0.65854491829355115987e0 * t18326 * t3689 + 0.26341796731742046394e1 * t2019 * t25113 * t2971 - 0.79025390195226139182e1 * t5718 * t25117 * t9671 - 0.13170898365871023197e1 * t2138 * t25122 * t2139;
    (t25113, t25117, t25121, t25122, t25126)
}
