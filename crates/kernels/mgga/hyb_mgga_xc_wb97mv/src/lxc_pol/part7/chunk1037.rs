//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1037/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1037<F: Float>(t2035: F, t4134: F, t6725: F, t1232: F, t8580: F, t3: F, t3177: F, t10545: F, t10550: F, t10554: F, t10559: F, t10564: F, t10574: F, t3155: F, t3157: F, t3159: F, t3167: F, t6717: F, t6743: F, t6747: F, t683: F, t686: F, t8528: F, t8530: F, t8536: F, t8546: F, t8548: F) -> (F, F, F) {
    let t10577 = t2035 * t6725 * t4134;
    let t10579 = t8580 * t1232;
    let t10583 = t3177 * t3;
    let t10587 = -t8546 - t3155 * t10545 * t3159 / 24.0 - 7.0 / 144.0 * t8528 * t8530 * t10550 + t3155 * t8536 * t10554 / 12.0 - t3155 * t3157 * t10559 / 24.0 - t3155 * t3157 * t10564 / 48.0 + t8548 * t3157 * t10550 / 16.0 + t6743 / 96.0 - t6747 + t6717 / 288.0 - t10574 / 192.0 - t10577 / 144.0 - t683 * t686 * t10579 / 32.0 + t683 * t3167 * t10583 / 16.0;
    (t10579, t10583, t10587)
}
