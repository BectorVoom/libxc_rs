//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 509/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk509<F: Float>(t684: F, t904: F, t2923: F, t2360: F, t327: F, t231: F, t2349: F, t1934: F, t893: F, t326: F) -> (F, F, F, F, F, F) {
    let t2924 = t684 * t904;
    let t2925 = t2923 * t2924;
    let t2928 = t327 * t2360;
    let t2930 = t231 * t2928 * t2349;
    let t2934 = t231 * t893 * t1934;
    let t2937 = t326 * t326;
    let t2938 = F::cast_from(1.0_f64) / t2937;
    (t2925, t2928, t2930, t2934, t2937, t2938)
}
