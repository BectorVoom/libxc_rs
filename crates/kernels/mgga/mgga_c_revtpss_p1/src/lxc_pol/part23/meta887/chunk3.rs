//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2804/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2804<F: Float>(t114: F, t46143: F, t46144: F, t49698: F, t49701: F, t49818: F, t75526: F, t75540: F, t75639: F, t75641: F, t75643: F, t75929: F, t116: F, t22746: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t75931 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t46143 + F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t46144 + F::cast_from(154.0_f64) / F::cast_from(9.0_f64) * t49698 + t49701 - t49818 + F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t75639 + F::cast_from(6.0_f64) * t75641 - F::cast_from(4.0_f64) * t75643 - F::cast_from(11.0_f64) / F::cast_from(3.0_f64) * t75540 - F::cast_from(2.0_f64) * t75526 + t75929);
    let t75941 = t22746 * t116;
    (t75931, t75941)
}
