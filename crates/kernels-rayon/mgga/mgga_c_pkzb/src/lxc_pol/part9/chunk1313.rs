//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1313/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1313(t2368: f64, t3026: f64, t8450: f64, t8452: f64, t926: f64, t300: f64, t3199: f64, t931: f64, t2099: f64, t8311: f64, t918: f64, t10047: f64, t18958: f64, t19014: f64, t23054: f64, t2380: f64, t3185: f64, t3187: f64, t3206: f64, t3207: f64, t6372: f64, t6373: f64, t6412: f64, t6479: f64, t6484: f64, t8254: f64, t8255: f64, t8260: f64, t8264: f64, t8435: f64) -> (f64, f64, f64) {
    let t23104 = t3026 * t2368;
    let t23122 = t8450 * t926 * t8452;
    let t23130 = t300 * t931 * t3199;
    let t23149 = t918 * t2099 * t8311;
    let t23163 = -0.68598428988911579154e-2_f64 * t10047 * t6373 - 0.51448821741683684367e-2_f64 * t3185 * t23130 * t8260 + 0.25724410870841842183e-2_f64 * t3206 * t23130 * t8255 + 0.12862205435420921092e-2_f64 * t3206 * t8254 * t18958 - 0.38586616306262763276e-2_f64 * t8435 * t23054 * t19014 * t3187 + 0.64311027177104605458e-3_f64 * t8450 * t23054 * t19014 * t3207 + 0.42874018118069736972e-3_f64 * t23149 - 0.38586616306262763276e-2_f64 * t3206 * t8264 * t6372 + 0.77173232612525526551e-2_f64 * t3185 * t8264 * t6412 + 0.38586616306262763276e-2_f64 * t2380 * t8264 * t6479 + 0.38586616306262763276e-2_f64 * t2380 * t8264 * t6484;
    (t23104, t23122, t23163)
}
