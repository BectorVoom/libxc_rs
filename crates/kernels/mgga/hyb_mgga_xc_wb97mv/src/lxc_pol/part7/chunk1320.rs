//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1320/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1320<F: Float>(t3563: F, t2572: F, t4354: F, t11424: F, t11425: F, t11440: F, t11461: F, t11464: F, t11467: F, t11470: F, t11532: F, t1416: F, t23395: F, t23438: F, t23440: F, t23450: F, t23459: F, t2534: F, t2535: F, t2550: F, t2556: F, t2558: F, t2575: F, t2589: F, t2595: F, t2597: F, t27399: F, t31730: F, t3582: F, t4332: F, t4333: F, t4349: F, t4376: F, t7259: F, t7328: F, t7333: F, t7360: F, t7421: F, t9492: F, t975: F, t976: F) -> (F,) {
    let t32252 = t3563 * t3563;
    let t32256 = t4354 * t2572;
    let t32299 = -0.20779030926817756511e3 * t23450 * t11440 - 0.10389515463408878255e3 * t7259 * t4376 * t2589 - 4.0 * t2534 * t32252 * t976 - 0.11696447245269292414e1 * t32256 * t2575 + 0.34631718211362927518e2 * t2595 * t31730 * t2597 + 0.64327917994770140268e2 * t2556 * t32252 * t2558 + 0.4138081033541872024e4 * t23395 * t11425 + 0.2069040516770936012e4 * t7360 * t11424 * t2550 + 0.19964560303604640732e6 * t23438 * t4332 * t23440 * t2535 - 0.46785788981077169656e1 * t27399 * t3582 + 12.0 * t7328 * t11461 + 6.0 * t2556 * t4333 * t2550 + 0.11579025239058625248e4 * t7360 * t4349 * t2535 - 8.0 * t7421 * t11464 - 4.0 * t2534 * t1416 * t9492 - 0.38596750796862084162e3 * t23459 * t11467 - 0.19298375398431042081e3 * t7333 * t4349 * t2550 - 4.0 * t7421 * t11470 - 4.0 * t2534 * t11532 * t975;
    (t32299,)
}
