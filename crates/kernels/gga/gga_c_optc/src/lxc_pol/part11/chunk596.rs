//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 596/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk596<F: Float>(t43: F, t277: F, t364: F, t4033: F, t4783: F, t4785: F, t4817: F, t4821: F, t4851: F, t4858: F, t4900: F, t4927: F, t5053: F, t5079: F, t95: F, t962: F, t4565: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t5080 = t4783 + t4785 + t4817 + t4821 + t4033 / 3.0 + t4851 * t364 / 2.0 + t4858 + t4927 + 0.25844881434903430496e-2 * t95 * t277 * t5053 * t962 - t4900 + t5079;
    let t5084 = piecewise3(t44, 0.0, t4565);
    (t5080, t5084)
}
