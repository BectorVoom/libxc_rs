//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1207/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1207<F: Float>(t1175: F, t8209: F, t10525: F, t8195: F, t10538: F, t546: F, t10517: F, t19: F, t549: F, t10532: F, t10296: F, t550: F, t1836: F, t3856: F, t10516: F, t10533: F, t1827: F, t1975: F, t25289: F, t25291: F, t25295: F, t26: F, t2966: F, t2967: F, t3131: F, t3135: F, t3967: F, t3969: F, t667: F, t8165: F, t8169: F, t8431: F) -> (F,) {
    let t29067 = t1175 * t8209;
    let t29069 = t8195 * t10525;
    let t29083 = t546 * t10538;
    let t29086 = t19 * t549 * t10517;
    let t29089 = t19 * t549 * t10532;
    let t29091 = t10296 * t550;
    let t29093 = t3856 * t1836;
    let t29095 = -t25289 / 32.0 + t25291 / 24.0 - 3.0 / 32.0 * t1175 * t8169 - 3.0 / 8.0 * t2966 * t2967 * t3135 - 3.0 / 32.0 * t1175 * t8431 - 3.0 / 16.0 * t1175 * t8165 - 3.0 / 8.0 * t2966 * t2967 * t3131 - t25295 / 16.0 - t29067 / 16.0 + 7.0 / 16.0 * t29069 - 3.0 / 32.0 * t546 * t10533 - 3.0 / 32.0 * t19 * t26 * t10516 * t667 - 3.0 / 64.0 * t19 * t26 * t3967 * t1827 - 3.0 / 64.0 * t1975 * t3969 - t29083 / 32.0 - t29086 / 32.0 - t29089 / 32.0 - t29091 / 32.0 - t29093 / 32.0;
    (t29095,)
}
