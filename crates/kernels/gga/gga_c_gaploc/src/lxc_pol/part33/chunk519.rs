//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 519/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk519<F: Float>(t188: F, t2822: F, t2792: F, t531: F, t2754: F, t569: F, t568: F, t1457: F, t2779: F, t2778: F, t475: F, t1445: F, t1000: F, t1004: F, t1008: F, t1013: F, t1456: F, t1580: F, t1599: F, t1641: F, t193: F, t2362: F, t2369: F, t2390: F, t2411: F, t2804: F, t2807: F, t2810: F, t2816: F, t2819: F, t541: F, t557: F, t574: F, t597: F) -> (F, F, F, F, F, F, F, F) {
    let t2823 = t188 * t2822;
    let t2828 = t531 * t2792;
    let t2833 = t569 * t2754;
    let t2834 = t568 * t2833;
    let t2843 = t1457 * t2779;
    let t2846 = t2778 * t475;
    let t2847 = t1445 * t2846;
    let t2850 = 0.30674340763136599741e1 * t597 * t2804 - 0.23833659967900284446e0 * t557 * t2807 - 0.30674340763136599741e1 * t574 * t2810 + 0.23833659967900284446e0 * t1000 * t541 + 0.23005755572352449806e1 * t597 * t2816 + 0.35750489951850426669e0 * t2819 * t193 + 0.35750489951850426669e0 * t2823 * t193 - 0.35750489951850426669e0 * t1599 * t1004 - 0.35750489951850426669e0 * t557 * t2828 - 0.23005755572352449806e1 * t1641 * t1008 - 0.23005755572352449806e1 * t574 * t2834 + 0.23005755572352449806e1 * t1580 * t1013 + 0.25561950635947166451e0 * t2362 - 0.29792074959875355558e-1 * t2369 - 0.59584149919750711116e-1 * t2390 + 0.29792074959875355558e-1 * t2411 + 0.35750489951850426669e0 * t1456 * t2843 - 0.46011511144704899612e1 * t574 * t2847;
    (t2823, t2828, t2833, t2834, t2843, t2846, t2847, t2850)
}
