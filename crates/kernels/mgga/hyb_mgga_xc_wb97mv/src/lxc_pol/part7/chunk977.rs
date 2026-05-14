//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 977/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk977<F: Float>(t3564: F, t975: F, t1416: F, t2550: F, t2535: F, t3567: F, t2558: F, t3563: F, t1415: F, t7362: F, t1428: F, t2574: F, t3597: F, t994: F, t2534: F, t2556: F, t2573: F, t2595: F, t3549: F, t3568: F, t3582: F, t3601: F, t7254: F, t7328: F, t7333: F, t7360: F, t7409: F, t7421: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9524 = t3564 * t975;
    let t9527 = t1416 * t2550;
    let t9530 = t3567 * t2535;
    let t9533 = t3563 * t2558;
    let t9534 = t9533 * t975;
    let t9537 = t3567 * t2550;
    let t9540 = t1415 * t7362;
    let t9541 = t9540 * t2535;
    let t9544 = t1428 * t2574;
    let t9547 = t1416 * t2535;
    let t9554 = t3597 * t994;
    let t9557 = -4.0 * t7421 * t3549 + 0.64327917994770140268e2 * t7328 * t3568 - 4.0 * t2534 * t9524 - 2.0 * t2534 * t9527 - 0.19298375398431042081e3 * t7333 * t9530 + 0.64327917994770140268e2 * t2556 * t9534 + 0.32163958997385070134e2 * t2556 * t9537 + 0.2069040516770936012e4 * t7360 * t9541 + 0.35089341735807877242e1 * t2595 * t9544 + 6.0 * t2556 * t9547 - 0.23392894490538584828e1 * t7409 * t3582 + 0.34631718211362927518e2 * t7254 * t3601 - 0.23392894490538584828e1 * t2573 * t9554;
    (t9524, t9527, t9530, t9534, t9537, t9541, t9544, t9547, t9554, t9557)
}
