//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1003/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1003<F: Float>(t513: F, t9988: F, t1522: F, t7907: F, t2849: F, t3718: F, t1148: F, t1520: F, t1528: F, t1531: F, t1534: F, t1538: F, t1544: F, t1547: F, t2839: F, t2853: F, t2860: F, t2869: F, t2873: F, t2887: F, t3697: F, t3725: F, t505: F, t511: F, t529: F, t7913: F, t7938: F, t8094: F, t9947: F, t9954: F, t9974: F, t9977: F, t9978: F, t9981: F, t9984: F, t9985: F) -> (F, F, F, F) {
    let t9989 = t9988 * t513;
    let t9992 = t7907 * t1522;
    let t9996 = t3718 * t2849;
    let t10003 = -24.0 * t511 * t9947 * t2849 + 120.0 * t7938 * t1544 * t2839 + 252.0 * t1148 * t9954 * t2849 - 180.0 * t2860 * t1547 * t2839 + 30.0 * t2860 * t1544 * t2869 - 36.0 * t1148 * t3697 * t2873 - 36.0 * t1148 * t1547 * t2869 - 0.16e-1 * t7913 * t1538 - 0.12e-1 * t8094 * t1534 + 400.0 / 27.0 * t9974 * t3725 + 504.0 * t9977 * t9978 + 24.0 * t9981 * t9978 - 360.0 * t9984 * t9985 + 0.6e-2 * t9989 * t1531 - 336.0 * t529 * t9992 * t2849 - 6.0 * t505 * t9996 + 2.0 * t1520 * t2853 - 4.0 * t2887 * t1528;
    (t9989, t9992, t9996, t10003)
}
