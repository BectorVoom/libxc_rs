//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1035/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1035<F: Float>(t10517: F, t26: F, t1226: F, t2967: F, t3856: F, t550: F, t1175: F, t3011: F, t3967: F, t667: F, t3968: F, t549: F, t19: F, t10296: F, t127: F, t2966: F, t3132: F, t3136: F, t3969: F, t546: F, t641: F, t669: F) -> (F, F, F, F, F, F) {
    let t10518 = t26 * t10517;
    let t10525 = t2967 * t1226;
    let t10528 = t3856 * t550;
    let t10530 = t1175 * t3011;
    let t10532 = t3967 * t667;
    let t10533 = t26 * t10532;
    let t10538 = t549 * t3968;
    let t10539 = t19 * t10538;
    let t10541 = -3.0 / 64.0 * t10296 * t127 - 3.0 / 64.0 * t3856 * t641 - 3.0 / 64.0 * t3856 * t669 - 3.0 / 64.0 * t19 * t10518 - 3.0 / 32.0 * t1175 * t3132 - 3.0 / 32.0 * t1175 * t3136 - 3.0 / 16.0 * t2966 * t10525 - t10528 / 64.0 - t10530 / 32.0 - 3.0 / 64.0 * t19 * t10533 - 3.0 / 64.0 * t546 * t3969 - t10539 / 64.0;
    (t10518, t10525, t10532, t10533, t10538, t10541)
}
