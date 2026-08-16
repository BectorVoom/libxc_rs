//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 398/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk398<F: Float>(t1936: F, t581: F, t144: F, t481: F, t152: F, t583: F, t6: F, t1524: F, t188: F, t178: F, t1: F, t172: F) -> (F, F, F, F, F, F, F) {
    let t1937 = t581 * t1936;
    let t1938 = t481 * t144;
    let t1939 = t1938 * t152;
    let t1940 = t583 * t6;
    let t1941 = t1939 * t1940;
    let t1944 = t1524 * t188;
    let t1945 = t178 * t1944;
    let t1946 = t172 * t1;
    (t1937, t1938, t1939, t1941, t1944, t1945, t1946)
}
