//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 988/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk988<F: Float>(t4300: F, t6571: F, t6553: F, t1880: F, t1902: F, t4142: F, t1492: F, t6624: F, t1519: F, t214: F) -> (F, F, F, F, F) {
    let t25216 = t6571 * t4300;
    let t25217 = t6553 * t25216;
    let t25218 = t1880 * t25217;
    let t25220 = t4142 * t1902;
    let t25222 = t1492 * t6624;
    let t25224 = t214 * t1519;
    (t25216, t25218, t25220, t25222, t25224)
}
