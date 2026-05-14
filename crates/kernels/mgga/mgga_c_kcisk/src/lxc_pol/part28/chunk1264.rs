//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1264/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1264<F: Float>(t1785: F, t8820: F, t2464: F, t7268: F, t8814: F, t24449: F, t642: F, t8672: F, t1692: F, t22919: F, t4822: F, t8616: F, t8939: F, t5531: F, t9258: F, t25136: F, t5439: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t68256 = t8820 * t1785;
    let t68510 = t2464 * t7268;
    let t69891 = t8814 * t1785;
    let t70785 = t24449 * sigma2;
    let t71037 = t642 * t8672;
    let t71223 = t22919 * t1692;
    let t71232 = t8616 * t4822;
    let t71399 = t642 * t8939;
    let t71472 = t9258 * t5531;
    let t73204 = t25136 * t5439;
    (t68256, t68510, t69891, t70785, t71037, t71223, t71232, t71399, t71472, t73204)
}
