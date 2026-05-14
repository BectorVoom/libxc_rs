//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1115/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1115<F: Float>(t1111: F, t1537: F, t3736: F, t3732: F, t1519: F, t646: F, t3742: F, t1298: F, t522: F, t1106: F, t11935: F, t4529: F, t1128: F, t1114: F, t1137: F, t2905: F, t4533: F, sigma0: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11943 = t1537 * t1111;
    let t11944 = t3736 * t11943;
    let t11951 = t3732 * t11943;
    let t11958 = t1519 * sigma0;
    let t11959 = t11958 * t646;
    let t11960 = t3742 * t11959;
    let t11963 = t1298 * t522;
    let t11964 = t1106 * tau1;
    let t11965 = t11963 * t11964;
    let t11968 = t3742 * t11935;
    let t11973 = t4529 * t1111;
    let t11974 = t1128 * t11973;
    let t11977 = t4529 * t1114;
    let t11978 = t1128 * t11977;
    let t11981 = t522 * t4529;
    let t11982 = t1137 * t11981;
    let t11985 = t4533 * t2905;
    (t11944, t11951, t11958, t11960, t11963, t11964, t11965, t11968, t11973, t11974, t11977, t11978, t11981, t11982, t11985)
}
