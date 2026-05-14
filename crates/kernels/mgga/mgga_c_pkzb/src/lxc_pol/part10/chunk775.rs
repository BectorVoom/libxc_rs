//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 775/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk775<F: Float>(t2127: F, t3679: F, t133: F, t3650: F, t793: F, t2139: F, t1138: F, t2123: F, t2138: F, t290: F, t2984: F, t3669: F, t791: F, t790: F, t1134: F, t1144: F, t307: F, t311: F, t3670: F, t3676: F) -> (F, F, F, F, F, F, F) {
    let t3680 = t3679 * t2127;
    let t3685 = t3650 * t133;
    let t3686 = t3685 * t793;
    let t3689 = t3679 * t2139;
    let t3694 = 0.13170898365871023197e1 * t2123 * t3680 + 0.13170898365871023197e1 * t2984 * t1138 + 0.65854491829355115987e0 * t791 * t3686 - 0.65854491829355115987e0 * t2138 * t3689 + 0.65854491829355115987e0 * t290 * t3669;
    let t3695 = t790 * t3694;
    let t3698 = 0.65854491829355115987e0 * t3670 * t311 - 0.13170898365871023197e1 * t1134 * t1144 + 0.13170898365871023197e1 * t307 * t3676 - 0.65854491829355115987e0 * t307 * t3695;
    (t3680, t3685, t3686, t3689, t3694, t3695, t3698)
}
