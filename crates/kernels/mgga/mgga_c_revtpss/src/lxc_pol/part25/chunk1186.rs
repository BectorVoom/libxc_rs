//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1186/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1186<F: Float>(t1444: F, t2022: F, t2030: F, t25921: F, t26034: F, t26046: F, t26079: F, t4003: F, t7295: F, t7296: F, t94413: F, t94641: F, t94643: F, t94648: F, t94650: F, t94656: F, t94662: F, t94665: F, t94672: F, t94675: F, t94677: F, t94682: F, t94683: F, t9658: F, t9994: F) -> (F,) {
    let t94692 = 0.38554277296572111609e-1 * t94641 - 0.4336814094102599731e0 * t94643 * t2030 + t94648 - 0.15421710918628844643e0 * t94650 + 0.26020884564615598386e1 * t7295 * t7296 * t26034 * t1444 + 0.10408353825846239354e2 * t7295 * t94656 * t2022 * t9658 + 0.57824187921367996415e-1 * t94662 - 0.43368140941025997312e-1 * t94665 - 0.23132566377943266966e0 * t94672 + 0.13010442282307799194e0 * t94675 + 0.51405703062096148812e-1 * t94677 + 0.13010442282307799193e1 * t25921 * t26046 + t94682 + 0.26020884564615598386e1 * t7295 * t94683 * t94413 * t9994 - 0.26020884564615598386e1 * t7295 * t26079 * t94413 * t4003;
    (t94692,)
}
