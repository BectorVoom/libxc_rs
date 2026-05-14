//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1116/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1116<F: Float>(t2909: F, t4533: F, t1111: F, t4541: F, t1114: F, t522: F, t1127: F, t1128: F, t1132: F, t1137: F, t11858: F, t11863: F, t11939: F, t11944: F, t11951: F, t11960: F, t11965: F, t11968: F, t11974: F, t11978: F, t11982: F, t11985: F, t2946: F, t2953: F, t3685: F, t7838: F, t7848: F, t7897: F, t7903: F, t7918: F, t7927: F, t8025: F, t8081: F, t8089: F, t9826: F, t9893: F) -> (F, F, F, F, F) {
    let t11988 = t4533 * t2909;
    let t11991 = t4541 * t1111;
    let t11995 = t4541 * t1114;
    let t11999 = t522 * t4541;
    let t12005 = -0.192e-3 * t7897 * t11944 + 0.288e-3 * t7838 * t11858 - 0.1408e-5 * t3685 * t11863 + 0.144e-2 * t7848 * t11951 - 0.1728e-2 * t7903 * t11939 - 0.1728e-2 * t7903 * t11944 - 0.512e-3 * t9893 * t11960 + 0.17777777777777777778e0 * t9893 * t11965 + 0.512e-3 * t9826 * t11968 + 0.17777777777777777778e0 * t9826 * t11965 + 0.36e-1 * t8089 * t11974 - 0.72e-1 * t7918 * t11978 - 0.48e-1 * t2946 * t11982 + 0.58666666666666666666e-1 * t1127 * t11985 - 0.58666666666666666666e-1 * t1132 * t11988 + 0.36e0 * t7927 * t1128 * t11991 - 0.54e0 * t8025 * t1128 * t11995 - 0.24e0 * t2953 * t1137 * t11999 + 0.756e0 * t8081 * t11974;
    (t11988, t11991, t11995, t11999, t12005)
}
