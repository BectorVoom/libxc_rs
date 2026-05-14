//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 907/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk907<F: Float>(t576: F, t8261: F, t3040: F, t35: F, t571: F, t6129: F, t6132: F, t6135: F, t6137: F, t6139: F, t6141: F, t8216: F, t8219: F, t8221: F, t8226: F, t8231: F, t8236: F, t8241: F, t8245: F, t8249: F, t8254: F, t8258: F) -> (F, F) {
    let t8262 = t8261 * t576;
    let t8266 = -t6132 - 4.0 / 243.0 * t6135 + t6137 / 243.0 - t6139 / 81.0 + t6141 / 162.0 - 2.0 / 243.0 * t8216 + t8219 - t8221 + 11.0 / 81.0 * t8226 - 5.0 / 243.0 * t571 * t8231 + 2.0 / 27.0 * t571 * t8236 - 4.0 / 81.0 * t3040 * t8241 - t571 * t8245 / 81.0 - t571 * t8249 / 9.0 + 4.0 / 27.0 * t3040 * t8254 + t571 * t8258 / 27.0 - t35 * t6129 * t8262 / 27.0;
    (t8262, t8266)
}
