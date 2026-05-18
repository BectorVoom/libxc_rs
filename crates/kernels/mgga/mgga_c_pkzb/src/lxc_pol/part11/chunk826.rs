//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 826/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk826<F: Float>(t3401: F, t51: F, t568: F, t6990: F, t2575: F, t2660: F, t2661: F, t3396: F, t1727: F, t3444: F, t3413: F, t5381: F) -> (F, F, F, F, F, F) {
    let t8821 = t51 * t3401;
    let t8823 = t6990 * t8821 * t568;
    let t8827 = t2660 * t2661 * t2575;
    let t8830 = t51 * t3396;
    let t8832 = t2660 * t8830 * t568;
    let t8835 = t1727 * t3444;
    let t8837 = t5381 * t3413;
    (t8821, t8823, t8827, t8832, t8835, t8837)
}
