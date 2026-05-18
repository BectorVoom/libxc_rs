//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 497/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk497<F: Float>(t2891: F, t473: F, t126: F, t507: F, t120: F, t1007: F, t518: F, t2880: F, t568: F, t1539: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t2892 = t473 * t2891;
    let t2894 = t126 * t507;
    let t2895 = t120 * t2894;
    let t2897 = t518 * t1007;
    let t2899 = t2880 * t568;
    let t2900 = t120 * t2899;
    let t2902 = t5 * t1539;
    (t2892, t2894, t2895, t2897, t2899, t2900, t2902)
}
