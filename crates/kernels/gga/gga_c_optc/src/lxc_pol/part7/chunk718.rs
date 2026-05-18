//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 718/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk718<F: Float>(t6807: F, t6808: F, t6812: F, t6841: F, t138: F, t2053: F, t637: F, t658: F, t120: F, t2086: F, t1928: F, t616: F) -> (F, F, F, F, F) {
    let t6843 = t6807 + t6808 + t6812 + t6841;
    let t6847 = t2053 * t138;
    let t6850 = t637 * t658;
    let t6855 = t120 * t2086;
    let t6856 = t1928 * t616;
    (t6843, t6847, t6850, t6855, t6856)
}
