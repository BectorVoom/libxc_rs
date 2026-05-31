//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1813/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1813<F: Float>(t114: F, t2089: F, t5920: F, t2055: F, t6765: F, t26148: F, t28034: F, t29999: F, t30001: F) -> (F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t30558 = t2089 * t5920;
    let t30563 = t6765 * t2055;
    let t30570 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t26148 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28034 + t29999 / F::cast_from(2.0_f64) - t30001 / F::cast_from(4.0_f64));
    (t30558, t30563, t30570)
}
