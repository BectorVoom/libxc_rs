//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1024/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1024(t3663: f64, t4711: f64, t2880: f64, t510: f64, t4714: f64, t521: f64, t2903: f64, t1139: f64, t1134: f64, t3747: f64, t3753: f64, t7806: f64, t7811: f64, t9504: f64, t9521: f64, t9535: f64, t9545: f64, t9552: f64, t9562: f64, t9568: f64, t9575: f64, t9587: f64, t9588: f64, t9594: f64, t9598: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9599 = t3663 * t4711;
    let t9602 = t2880 * tau0;
    let t9603 = t510 * t9602;
    let t9604 = t3663 * t4714;
    let t9607 = t521 * tau0;
    let t9608 = t2903 * t9607;
    let t9611 = t1139 * tau0;
    let t9612 = t1134 * t9611;
    let t9617 = 32.0_f64 * t7806 * t9568 + 32.0_f64 / 9.0_f64 * t7811 * t9568 + 700.0_f64 / 3.0_f64 * t9575 * t9535 + 32.0_f64 / 9.0_f64 * t7811 * t9545 + 200.0_f64 / 9.0_f64 * t9521 * t9535 + 32.0_f64 / 9.0_f64 * t7811 * t9552 - 64.0_f64 / 27.0_f64 * t3747 * t9504 - 512.0_f64 / 729.0_f64 * t9587 * t9588 - 128.0_f64 / 81.0_f64 * t3753 * t9562 - 512.0_f64 / 729.0_f64 * t9594 * t9588 - 400.0_f64 / 9.0_f64 * t9598 * t9599 + 200.0_f64 / 3.0_f64 * t9603 * t9604 - 1000.0_f64 / 3.0_f64 * t9608 * t9599 + 400.0_f64 * t9612 * t9604 - 400.0_f64 * t9612 * t9599;
    (t9602, t9603, t9604, t9608, t9611, t9612, t9617)
}
