//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1431/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1431<F: Float>(t12626: F, t1769: F, t487: F, t12627: F, t1811: F, t11239: F, t1770: F, t13061: F, t13051: F, t12909: F, t17395: F, t3781: F, t5219: F, t5330: F) -> (F, F, F, F, F, F, F, F) {
    let t56331 = t1769 * t12626;
    let t56332 = t56331 * t487;
    let t56393 = t12627 * t1811;
    let t56730 = t1770 * t11239;
    let t56731 = t56730 * t13061;
    let t57065 = t56730 * t13051;
    let t57147 = t12909 * t17395;
    let t57382 = t5219 * t3781 * t5330;
    (t56331, t56332, t56393, t56730, t56731, t57065, t57147, t57382)
}
