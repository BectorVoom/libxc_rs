//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 927/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk927<F: Float>(t38: F, t4217: F, t1469: F, t2299: F, t4186: F, t633: F, t2306: F, t637: F, t606: F, t77: F) -> (F, F, F, F) {
    let t4218 = t38 * t4217;
    let t4227 = t2299 * t1469;
    let t4230 = t633 * t4186;
    let t4232 = t2306 * t1469;
    let t4235 = t637 * t4186;
    let t4237 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4227 * t606 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4230 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4232 * t606 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4235;
    let t4238 = t77 * t4237;
    (t4218, t4227, t4232, t4238)
}
