//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1043/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1043<F: Float>(t3652: F, t5939: F, t757: F, t2887: F, t68: F, t9554: F, t9297: F, t9301: F, t5931: F, t9685: F, t751: F, t9633: F, t3650: F, t785: F, t2036: F, t25113: F) -> (F, F, F, F, F, F, F, F) {
    let t26535 = t757 * t5939 * t3652;
    let t26585 = t2887 * t68 * t9554;
    let t26588 = t2887 * t68 * t9297;
    let t26592 = t2887 * t68 * t9301;
    let t26646 = t5931 * t9685;
    let t26653 = t751 * t9633;
    let t26659 = t785 * t3650;
    let t26667 = t2036 * t25113;
    (t26535, t26585, t26588, t26592, t26646, t26653, t26659, t26667)
}
