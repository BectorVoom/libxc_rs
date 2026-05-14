//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1266/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1266<F: Float>(t2198: F, t4157: F, t2201: F, t2245: F, t2247: F, t30989: F, t11097: F, t22432: F, t2263: F, t2296: F, t2302: F, t26624: F, t30948: F, t30961: F, t30963: F, t30965: F, t30967: F, t30970: F, t30973: F, t30975: F, t30977: F, t30979: F, t30992: F, t4203: F, t4230: F, t4243: F, t6876: F, t6972: F, t6982: F, t9177: F) -> (F, F, F) {
    let t30993 = t4157 * t2198;
    let t30995 = 2.0 * t30993 * t2201;
    let t30998 = 0.32163958997385070134e2 * t2245 * t30989 * t2247;
    let t31002 = -t30948 - 0.77193501593724168323e3 * t26624 * t9177 + t30961 - t30963 + t30965 - t30967 - t30970 - t30973 - t30975 - t30977 - t30979 - 0.11696447245269292414e1 * t22432 * t4230 + 0.5848223622634646207e0 * t6972 * t4243 + 0.11696447245269292414e1 * t2296 * t11097 - 0.14035736694323150897e2 * t6982 * t4230 * t2302 + t30992 + t30995 - t30998 - 24.0 * t6876 * t4203 * t2263;
    (t30995, t30998, t31002)
}
