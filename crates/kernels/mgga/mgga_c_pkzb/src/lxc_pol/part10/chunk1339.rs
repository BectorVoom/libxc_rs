//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1339/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1339<F: Float>(t2989: F, t1134: F, t1144: F, t158: F, t2112: F, t2118: F, t2145: F, t26643: F, t2957: F, t2990: F, t307: F, t311: F, t3695: F, t7805: F, t7828: F, t786: F, t7885: F, t799: F, t800: F, t9634: F, t9647: F, t9651: F, t9656: F, t9712: F, t9713: F) -> (F,) {
    let t26742 = t2989 * t2989;
    let t26775 = 0.26341796731742046394e1 * t307 * t2118 * t26742 - 0.13170898365871023197e1 * t1134 * t7885 - 0.13170898365871023197e1 * t9634 * t800 + 0.52683593463484092788e1 * t786 * t9651 - 0.65854491829355115987e0 * t2112 * t3695 - 0.13170898365871023197e1 * t786 * t9713 + 0.26341796731742046394e1 * t307 * t2118 * t9712 * t799 - 0.26341796731742046394e1 * t2957 * t2990 - 0.13170898365871023197e1 * t7805 * t1144 + 0.65854491829355115987e0 * t26643 * t158 * t311 + 0.13170898365871023197e1 * t307 * t9656 * t2145 + 0.26341796731742046394e1 * t1134 * t7828 - 0.39512695097613069591e1 * t307 * t9647 * t2145;
    (t26775,)
}
