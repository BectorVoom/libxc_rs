//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 516/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk516<F: Float>(t114: F, t1916: F, t1918: F, t572: F, t573: F, t198: F, t207: F, t159: F, t215: F, t104: F, t655: F, t109: F, t69: F) -> (F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t1921 = t1916 * t573 + 3.0 * t1918 * t572;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2194 = t655 * t104;
    let t2195 = t2194 * t109;
    let t2198 = piecewise3(t115, 0.0, -t69 * t2195 / 8.0);
    (t1921, t1940, t1941, t2194, t2195, t2198)
}
