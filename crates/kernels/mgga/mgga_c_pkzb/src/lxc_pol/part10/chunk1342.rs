//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1342/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1342<F: Float>(t1306: F, t2149: F, t25668: F, t25674: F, t25676: F, t25678: F, t25680: F, t25691: F, t25695: F, t25697: F, t25699: F, t25701: F, t25703: F, t25816: F, t2997: F, t7888: F, t9725: F) -> (F,) {
    let t26797 = -t1306 * t2149 * t9725 - 2.0 * t1306 * t2997 * t7888 + t25668 - t25674 - t25676 - t25678 - t25680 - t25691 - t25695 - t25697 - t25699 - t25701 + t25703 + t25816;
    (t26797,)
}
