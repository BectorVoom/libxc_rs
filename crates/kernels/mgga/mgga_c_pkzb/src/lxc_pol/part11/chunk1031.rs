//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1031/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1031<F: Float>(t1639: F, t8770: F, t1667: F, t8717: F, t501: F, t8775: F, t8777: F, t46: F, t552: F, t8748: F, t1545: F, t3426: F, t1769: F, t8832: F, t1044: F, t5389: F) -> (F, F, F, F, F, F, F, F) {
    let t24606 = t8770 * t1639;
    let t24642 = t8717 * t1667;
    let t24651 = t501 * t8775;
    let t24653 = t501 * t8777;
    let t24662 = t8748 * t46 * t552;
    let t24671 = t1545 * t3426;
    let t24729 = t1769 * t8832;
    let t24792 = t5389 * t1044;
    (t24606, t24642, t24651, t24653, t24662, t24671, t24729, t24792)
}
