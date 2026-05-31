//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1024/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1024<F: Float>(t3663: F, t4711: F, t2880: F, t510: F, t4714: F, t521: F, t2903: F, t1139: F, t1134: F, t3747: F, t3753: F, t7806: F, t7811: F, t9504: F, t9521: F, t9535: F, t9545: F, t9552: F, t9562: F, t9568: F, t9575: F, t9587: F, t9588: F, t9594: F, t9598: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t9599 = t3663 * t4711;
    let t9602 = t2880 * tau0;
    let t9603 = t510 * t9602;
    let t9604 = t3663 * t4714;
    let t9607 = t521 * tau0;
    let t9608 = t2903 * t9607;
    let t9611 = t1139 * tau0;
    let t9612 = t1134 * t9611;
    let t9617 = F::cast_from(32.0_f64) * t7806 * t9568 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t9568 + F::cast_from(700.0_f64) / F::cast_from(3.0_f64) * t9575 * t9535 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t9545 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t9521 * t9535 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t9552 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3747 * t9504 - F::cast_from(512.0_f64) / F::cast_from(729.0_f64) * t9587 * t9588 - F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t3753 * t9562 - F::cast_from(512.0_f64) / F::cast_from(729.0_f64) * t9594 * t9588 - F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t9598 * t9599 + F::cast_from(200.0_f64) / F::cast_from(3.0_f64) * t9603 * t9604 - F::cast_from(1000.0_f64) / F::cast_from(3.0_f64) * t9608 * t9599 + F::cast_from(400.0_f64) * t9612 * t9604 - F::cast_from(400.0_f64) * t9612 * t9599;
    (t9602, t9603, t9604, t9608, t9611, t9612, t9617)
}
