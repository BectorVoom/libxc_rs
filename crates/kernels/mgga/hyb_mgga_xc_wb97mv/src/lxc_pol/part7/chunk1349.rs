//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1349/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1349<F: Float>(t1126: F, t1298: F, t9825: F, t1291: F, t2952: F, t3740: F, t11772: F, t7833: F, t11821: F, t7899: F, t11826: F, t9831: F, t24605: F, t28356: F, t28388: F, t28392: F, t28400: F, t28833: F, t32670: F, t32798: F, t32804: F, t32827: F, t33007: F, t4619: F, t7838: F, t7897: F, t9832: F, t9835: F, t9846: F, t9879: F) -> (F, F, F, F, F) {
    let t33021 = t1126 * t9825 * t1298;
    let t33029 = t2952 * t3740 * t1291;
    let t33033 = t2952 * t3740 * t1298;
    let t33038 = t7833 * t11772;
    let t33041 = t11821 * t7899;
    let t33046 = t11826 * t9831;
    let t33049 = t11826 * t7899;
    let t33052 = -0.1728e-1 * t28400 * t4619 * t9831 - 0.110592e-6 * t28356 * t33007 + 0.1152e-2 * t28833 * t32827 + 0.2048e-2 * t28392 * t9835 - 0.71111111111111111112e0 * t33021 * t32798 + 0.2048e-2 * t28392 * t9846 + 0.71111111111111111112e0 * t33021 * t32804 - 0.1536e-1 * t33029 * t9832 + 0.53333333333333333333e1 * t33033 * t32804 + 0.18432e-1 * t28388 * t9835 - 0.8064e1 * t24605 * t33038 - 0.192e-3 * t7897 * t33041 - 0.47407407407407407408e0 * t9879 * t32670 - 0.192e-3 * t7897 * t33046 + 0.288e-3 * t7838 * t33049;
    (t33038, t33041, t33046, t33049, t33052)
}
