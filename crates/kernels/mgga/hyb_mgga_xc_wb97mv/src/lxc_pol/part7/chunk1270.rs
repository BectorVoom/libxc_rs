//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1270/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1270<F: Float>(t11146: F, t11149: F, t11152: F, t11174: F, t1353: F, t22445: F, t2262: F, t2263: F, t22758: F, t2278: F, t2284: F, t26934: F, t31132: F, t31135: F, t31138: F, t31140: F, t31143: F, t31146: F, t31149: F, t31151: F, t4203: F, t4219: F, t6871: F, t6876: F, t6903: F, t6923: F, t9049: F, t9061: F, t9180: F, t9184: F) -> (F,) {
    let t31152 = 0.64327917994770140268e2 * t9061 * t9180 + 0.4138081033541872024e4 * t26934 * t9184 + 12.0 * t6871 * t11146 + 6.0 * t2284 * t4203 * t2278 + 0.11579025239058625248e4 * t6903 * t4219 * t2263 - 8.0 * t6923 * t11149 - 4.0 * t2262 * t1353 * t9049 - 0.38596750796862084162e3 * t22758 * t11152 - 0.19298375398431042081e3 * t6876 * t4219 * t2278 - 0.24828486201251232145e5 * t22445 * t11174 * t2263 + t31132 + t31135 + t31138 + t31140 + t31143 + t31146 + t31149 - t31151;
    (t31152,)
}
