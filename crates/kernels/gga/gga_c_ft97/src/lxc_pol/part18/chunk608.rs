//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 608/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk608<F: Float>(t39: F, t550: F, t133: F, t542: F, t2044: F, t7853: F, t131: F, t1991: F, t139: F, t548: F, t135: F, t8078: F, t120: F, t1655: F, t40: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8851 = t550 * t39;
    let t8852 = t133 * t8851;
    let t8859 = t542 * t8851;
    let t8885 = t7853 * t2044;
    let t8894 = t1991 * t131;
    let t8895 = t8894 * t139;
    let t8906 = t548 * t548;
    let t8907 = 1.0 / t8906;
    let t8908 = t135 * t8907;
    let t8914 = 0.18521666970164609055e-1 * t8078;
    let t8942 = t120 * t1655;
    let t8946 = t6 / t40;
    (t8851, t8852, t8859, t8885, t8894, t8895, t8906, t8907, t8908, t8914, t8942, t8946)
}
