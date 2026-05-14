//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 372/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk372<F: Float>(t1810: F, t572: F, t1767: F, t1770: F, t1773: F, t1777: F, t1779: F, t1782: F) -> (F, F) {
    let t1811 = t1810 * t572;
    let t1820 = -0.78438333333333333333e0 * t1767 + 0.15687666666666666667e1 * t1770 + 0.68863333333333333333e0 * t1773 + 0.14025833333333333333e0 * t1777 + 0.28051666666666666667e0 * t1779 + 0.17365833333333333333e0 * t1782;
    (t1811, t1820)
}
