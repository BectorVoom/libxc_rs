//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 498/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk498<F: Float>(t684: F, t904: F, t2923: F, t2360: F, t327: F, t231: F, t2349: F, t1934: F, t893: F, t326: F, t898: F, t2400: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2924 = t684 * t904;
    let t2925 = t2923 * t2924;
    let t2928 = t327 * t2360;
    let t2930 = t231 * t2928 * t2349;
    let t2934 = t231 * t893 * t1934;
    let t2937 = t326 * t326;
    let t2938 = F::new(1.0) / t2937;
    let t2939 = t904 * t904;
    let t2941 = t898 * t2938 * t2939;
    let t2946 = F::new(0.19257444444444444444e0) * t2400;
    (t2924, t2925, t2930, t2934, t2937, t2938, t2939, t2941, t2946)
}
