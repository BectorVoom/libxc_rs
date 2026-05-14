//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1207/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1207<F: Float>(t10460: F, t677: F, t10433: F, t136: F, t550: F, t2011: F, t3990: F, t1815: F, t3985: F, t20216: F, t2024: F, t3926: F, t1238: F, t2027: F, t23804: F, t23975: F, t23977: F, t23985: F, t23987: F, t23990: F, t23992: F, t23994: F, t23996: F, t23999: F, t24455: F, t3: F, t3150: F, t3925: F, t6457: F, t684: F, t687: F, t8492: F) -> (F,) {
    let t28046 = t677 * t10460;
    let t28049 = t136 * t550 * t10433;
    let t28057 = t3990 * t2011;
    let t28060 = t136 * t1815 * t3985;
    let t28066 = t2024 * t20216 * t3926;
    let t28084 = -t28046 / 32.0 - t28049 / 32.0 - 5.0 / 144.0 * t23975 + t23977 / 24.0 + t23985 / 48.0 + t23987 / 24.0 - 5.0 / 144.0 * t23990 - t23992 / 32.0 + t28057 / 96.0 + t28060 / 96.0 - t23994 / 32.0 - t23996 / 16.0 + t23999 / 24.0 + t28066 / 216.0 - t2024 * t2027 * t6457 * t3925 / 48.0 - t684 * t687 * t23804 * t1238 / 32.0 + t684 * t3150 * t8492 * t3 / 8.0 - t684 * t687 * t24455 * t1238 / 32.0;
    (t28084,)
}
