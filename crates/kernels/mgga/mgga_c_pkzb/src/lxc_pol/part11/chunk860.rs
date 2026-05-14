//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 860/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk860<F: Float>(t6090: F, t6211: F, t7955: F, t8076: F, t9772: F, t9774: F, t9777: F, t9782: F, t9797: F, t9799: F, t9806: F, t9808: F, t6177: F, t6218: F, t7950: F, t8090: F, t8091: F, t9812: F, t9814: F, t9819: F, t9823: F, t9826: F, t9830: F, t9834: F) -> (F, F) {
    let t9918 = 0.19419375e1 * t9772 - 0.258925e1 * t9774 - 0.1294625e1 * t9777 + 0.258925e1 * t9799 - t6211 + 0.40256666666666666667e0 * t6090 + 0.80513333333333333333e0 * t7955 - t8076 - 0.301925e0 * t9782 + 0.905775e0 * t9797 - 0.412621875e-1 * t9806 + 0.16504875e0 * t9808;
    let t9928 = 0.82524375e-1 * t9812 + 0.16504875e0 * t9814 - t6218 + 0.27595e0 * t6177 + 0.5519e0 * t7950 - t8090 - t8091 - 0.16557e0 * t9819 + 0.49671e0 * t9823 - 0.16557e0 * t9826 + 0.248355e0 * t9830 + 0.248355e0 * t9834;
    (t9918, t9928)
}
