//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 907/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk907<F: Float>(t10305: F, t794: F, t188: F, t297: F, t818: F, t2531: F, t799: F, t2493: F, t435: F, t3243: F, t2316: F, t493: F) -> (F, F, F, F) {
    let t10306 = t794 * t10305;
    let t10309 = t188 * t818 * t297;
    let t10310 = t10309 * t2531;
    let t10311 = t799 * t10310;
    let t10313 = t435 * t2493;
    let t10314 = t3243 * t10313;
    let t10316 = t493 * t2316;
    (t10306, t10311, t10314, t10316)
}
