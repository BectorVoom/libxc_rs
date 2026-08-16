//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 524/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk524<F: Float>(t114: F, t1916: F, t1918: F, t572: F, t573: F, t198: F, t207: F, t159: F, t215: F, t655: F, t96: F, t101: F, t69: F) -> (F, F, F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t1921 = t1916 * t573 + F::cast_from(3.0_f64) * t1918 * t572;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2174 = t655 * t96;
    let t2175 = t2174 * t101;
    let t2178 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t69 * t2175 / F::cast_from(8.0_f64));
    (t1921, t1940, t1941, t2174, t2175, t2178)
}
