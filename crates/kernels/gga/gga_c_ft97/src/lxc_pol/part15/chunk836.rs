//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 836/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk836<F: Float>(t1882: F, t20270: F, t20463: F, t376: F, t89: F, t20233: F, t8392: F, t20230: F, t20240: F, t20292: F, t20307: F, t103: F, t20098: F, t20409: F, t20288: F, t1526: F, t4422: F, t7705: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t75531 = t1882 * t20270;
    let t75584 = t89 * t376 * t20463;
    let t75586 = t8392 * t20233;
    let t75588 = t8392 * t20230;
    let t75590 = t8392 * t20240;
    let t75624 = t8392 * t20292;
    let t75642 = t1882 * t20307;
    let t75678 = t103 * t20098;
    let t75766 = t1882 * t20409;
    let t75845 = t8392 * t20288;
    let t75878 = t1526 * t7705 * t4422;
    (t75531, t75584, t75586, t75588, t75590, t75624, t75642, t75678, t75766, t75845, t75878)
}
