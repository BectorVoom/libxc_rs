//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1354/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1354<F: Float>(t1937: F, t49856: F, t18163: F, t6993: F, t25188: F, t7239: F, t46126: F, t49851: F, t10416: F, t25081: F, t7234: F, t25083: F) -> (F, F, F, F, F, F, F) {
    let t95073 = F::cast_from(2.0_f64) * t49856 * t1937;
    let t95075 = F::cast_from(6.0_f64) * t18163 * t6993;
    let t95081 = F::cast_from(9.0_f64) * t25188 * t7239;
    let t95083 = F::cast_from(2.0_f64) * t46126 * t1937;
    let t95085 = F::cast_from(6.0_f64) * t49851 * t1937;
    let t95087 = F::cast_from(6.0_f64) * t10416 * t6993;
    let t95088 = t7234 * t25081;
    let t95090 = F::cast_from(18.0_f64) * t95088 * t25083;
    (t95073, t95075, t95081, t95083, t95085, t95087, t95090)
}
