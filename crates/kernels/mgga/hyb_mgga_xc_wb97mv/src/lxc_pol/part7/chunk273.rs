//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 273/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk273<F: Float>(t122: F, t319: F, t120: F, t317: F, t322: F, t326: F, t304: F, t328: F, t332: F, t325: F, t324: F, t301: F, t101: F, t298: F, t299: F, t309: F, t314: F, t318: F, t327: F, t660: F, t664: F, t868: F, t869: F, t873: F, t879: F, t882: F, t887: F, rho0: F, tau0: F) -> (F, F, F, F, F, F, F, F) {
    let t890 = t319 * t122;
    let t891 = 1.0 / t890;
    let t892 = t891 * tau0;
    let t897 = t322 * t317 * t120;
    let t898 = t897 * t326;
    let t899 = t328 * t304;
    let t900 = t332 * tau0;
    let t901 = t899 * t900;
    let t905 = 1.0 / t325 / t122;
    let t906 = t324 * t905;
    let t909 = t301 * rho0;
    let t910 = 1.0 / t909;
    let t919 = -0.17066666666666666667e-1 * t299 * t869 + 0.34133333333333333333e-2 * t873 * t879 + 5.0 / 3.0 * t882 * t660 + 5.0 / 3.0 * t314 * t664 + 10.0 / 3.0 * t887 * t664 + 10.0 / 3.0 * t318 * t892 * t101 + 0.53333333333333333333e-1 * t898 * t901 + 0.53333333333333333333e-1 * t906 * t901 - 0.64e-1 * t327 * t328 * t910 * t332 + 0.128e-1 * t327 * t298 * t868 * t309;
    (t891, t892, t897, t898, t900, t905, t906, t919)
}
