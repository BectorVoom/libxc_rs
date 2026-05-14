//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1409/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1409<F: Float>(t170: F, t60: F, t7028: F, t5300: F, t963: F, t5286: F, t2768: F, t5251: F, t5250: F, t2482: F, t5249: F, t5252: F, t1651: F, t22313: F, t22315: F, t22319: F, t22321: F, t22323: F, t22325: F, t22329: F, t2769: F, t5367: F, t596: F, t7761: F) -> (F,) {
    let t26664 = t60 * t7028 * t170;
    let t26667 = t963 * t5300;
    let t26669 = t963 * t5286;
    let t26671 = t2768 * t5251;
    let t26672 = t5250 * t26671;
    let t26673 = 0.12154685976e1 * t26672;
    let t26675 = t5249 * t2482 * t5252;
    let t26676 = 0.12154685976e1 * t26675;
    let t26677 = -0.31168546390226634767e3 * t22313 + 0.31580407562227089518e2 * t22315 + t22319 + 0.4051561992e0 * t22321 + 0.10389515463408878255e3 * t22323 - 0.10526802520742363173e2 * t22325 + 0.15584273195113317383e3 * t22329 - 0.675260332e-1 * t5367 * t2769 - 0.2025780996e0 * t1651 * t7761 - 0.2025780996e0 * t596 * t26664 + 0.6233709278045326953e3 * t26667 - 0.14035736694323150897e2 * t26669 - t26673 - t26676;
    (t26677,)
}
