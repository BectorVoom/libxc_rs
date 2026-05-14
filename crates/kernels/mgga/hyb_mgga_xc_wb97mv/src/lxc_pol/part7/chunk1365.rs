//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1365/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1365<F: Float>(t2873: F, t4619: F, t1114: F, t12058: F, t11973: F, t2856: F, t3803: F, t1128: F, t11761: F, t11772: F, t11991: F, t12050: F, t12072: F, t27886: F, t27890: F, t2869: F, t28787: F, t2931: F, t2953: F, t4550: F, t4551: F, t9764: F, t9977: F, t9981: F, t9984: F, t9989: F) -> (F,) {
    let t33649 = t4619 * t2873;
    let t33661 = t12058 * t1114;
    let t33666 = t11973 * t1114;
    let t33669 = t3803 * t2856;
    let t33682 = 0.6e-2 * t9989 * t4551 - 0.12e-1 * t11761 * t2931 + 24.0 * t9981 * t33649 - 360.0 * t9984 * t4619 * t2869 - 1440.0 * t28787 * t11991 * t1114 - 360.0 * t9984 * t12072 * t1114 + 504.0 * t9977 * t33661 + 24.0 * t9981 * t33661 - 4032.0 * t27886 * t33666 + 48.0 * t33669 * t11772 - 96.0 * t27890 * t33666 + 504.0 * t9977 * t33649 + 0.9e-1 * t2953 * t1128 * t4550 * t2869 - 0.24e-1 * t9764 * t12050;
    (t33682,)
}
