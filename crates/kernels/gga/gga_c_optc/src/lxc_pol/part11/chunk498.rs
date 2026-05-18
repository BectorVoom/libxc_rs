//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 498/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk498<F: Float>(t1274: F, t2030: F, t127: F, t1271: F, t1235: F, t1884: F, t1239: F, t1896: F, t1244: F, t75: F) -> (F, F, F, F, F) {
    let t3358 = t2030 * t1274;
    let t3360 = t1271 * t127;
    let t3365 = t1884 * t1235;
    let t3373 = t1896 * t1239;
    let t3386 = t1244 * t75;
    (t3358, t3360, t3365, t3373, t3386)
}
