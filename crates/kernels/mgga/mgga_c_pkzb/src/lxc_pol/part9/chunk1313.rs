//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1313/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1313<F: Float>(t2368: F, t3026: F, t8450: F, t8452: F, t926: F, t300: F, t3199: F, t931: F, t2099: F, t8311: F, t918: F, t10047: F, t18958: F, t19014: F, t23054: F, t2380: F, t3185: F, t3187: F, t3206: F, t3207: F, t6372: F, t6373: F, t6412: F, t6479: F, t6484: F, t8254: F, t8255: F, t8260: F, t8264: F, t8435: F) -> (F, F, F) {
    let t23104 = t3026 * t2368;
    let t23122 = t8450 * t926 * t8452;
    let t23130 = t300 * t931 * t3199;
    let t23149 = t918 * t2099 * t8311;
    let t23163 = -F::new(0.68598428988911579154e-2) * t10047 * t6373 - F::new(0.51448821741683684367e-2) * t3185 * t23130 * t8260 + F::new(0.25724410870841842183e-2) * t3206 * t23130 * t8255 + F::new(0.12862205435420921092e-2) * t3206 * t8254 * t18958 - F::new(0.38586616306262763276e-2) * t8435 * t23054 * t19014 * t3187 + F::new(0.64311027177104605458e-3) * t8450 * t23054 * t19014 * t3207 + F::new(0.42874018118069736972e-3) * t23149 - F::new(0.38586616306262763276e-2) * t3206 * t8264 * t6372 + F::new(0.77173232612525526551e-2) * t3185 * t8264 * t6412 + F::new(0.38586616306262763276e-2) * t2380 * t8264 * t6479 + F::new(0.38586616306262763276e-2) * t2380 * t8264 * t6484;
    (t23104, t23122, t23163)
}
