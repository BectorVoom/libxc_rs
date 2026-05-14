//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 380/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk380<F: Float>(t103: F, t133: F, t193: F, t197: F, t102: F, t745: F, t48: F, t53: F, t539: F, t592: F, t544: F, t171: F, t110: F, t518: F, t84: F, t596: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1923 = 1100.0 / 81.0 * t193 * t133 * t103 * t197;
    let t1924 = t745 * t102;
    let t1933 = 1.0 / t48;
    let t1940 = 1.0 / t53;
    let t1966 = 8.0 * t539 * t592;
    let t1968 = 8.0 * t544 * t592;
    let t1974 = t171 * t171;
    let t1975 = 1.0 / t1974;
    let t1983 = t518 * t110 * t84;
    let t1985 = 0.24415406715670879921e-3 * t596 * t1983;
    (t1923, t1924, t1933, t1940, t1966, t1968, t1974, t1975, t1983, t1985)
}
