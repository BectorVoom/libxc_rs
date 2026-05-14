//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 983/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk983<F: Float>(t3663: F, t4711: F, t2880: F, t510: F, t4714: F, t521: F, t2903: F, t1139: F, t1134: F, t3747: F, t3753: F, t7806: F, t7811: F, t9504: F, t9521: F, t9535: F, t9545: F, t9552: F, t9562: F, t9568: F, t9575: F, t9587: F, t9588: F, t9594: F, t9598: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t9599 = t3663 * t4711;
    let t9602 = t2880 * tau0;
    let t9603 = t510 * t9602;
    let t9604 = t3663 * t4714;
    let t9607 = t521 * tau0;
    let t9608 = t2903 * t9607;
    let t9611 = t1139 * tau0;
    let t9612 = t1134 * t9611;
    let t9617 = 32.0 * t7806 * t9568 + 32.0 / 9.0 * t7811 * t9568 + 700.0 / 3.0 * t9575 * t9535 + 32.0 / 9.0 * t7811 * t9545 + 200.0 / 9.0 * t9521 * t9535 + 32.0 / 9.0 * t7811 * t9552 - 64.0 / 27.0 * t3747 * t9504 - 512.0 / 729.0 * t9587 * t9588 - 128.0 / 81.0 * t3753 * t9562 - 512.0 / 729.0 * t9594 * t9588 - 400.0 / 9.0 * t9598 * t9599 + 200.0 / 3.0 * t9603 * t9604 - 1000.0 / 3.0 * t9608 * t9599 + 400.0 * t9612 * t9604 - 400.0 * t9612 * t9599;
    (t9602, t9603, t9604, t9608, t9611, t9612, t9617)
}
