//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 600/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk600<F: Float>(t128: F, t39: F, t2035: F, t1995: F, t2031: F, t554: F, t7883: F, t1701: F, t2058: F, t6: F, t133: F, t1702: F, t2059: F, t542: F, t2071: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8811 = t128 * t39;
    let t8812 = t8811 * t2035;
    let t8825 = t1995 * t2031;
    let t8828 = t7883 * t554;
    let t8829 = t1701 * t8828;
    let t8832 = t2058 * t6;
    let t8833 = t133 * t8832;
    let t8835 = t1701 * t1702 * t2059;
    let t8838 = t542 * t8832;
    let t8847 = t1702 * t2071;
    let t8848 = t1701 * t8847;
    let t8851 = t550 * t39;
    (t8811, t8812, t8825, t8829, t8832, t8833, t8835, t8838, t8848, t8851)
}
