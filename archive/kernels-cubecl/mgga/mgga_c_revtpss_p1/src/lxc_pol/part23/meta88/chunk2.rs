//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 615/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk615<F: Float>(t1916: F, t1918: F, t572: F, t573: F, t76: F, t84: F, t198: F, t207: F) -> (F, F, F) {
    let t1921 = t1916 * t573 + F::cast_from(3.0_f64) * t1918 * t572;
    let t1927 = t76 * t84;
    let t1940 = t198 * t207;
    (t1921, t1927, t1940)
}
