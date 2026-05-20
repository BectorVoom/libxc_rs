//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2094/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2094<F: Float>(t29010: F, t3704: F, t17720: F, t7624: F, t15904: F, t26865: F, t13127: F, t17400: F, t26866: F, t1802: F, t3089: F, t3717: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t104689 = F::cast_from(0.57165357490759649296e-3_f64) * t29010 * t3704;
    let t104691 = F::cast_from(0.6351706387862183255e-3_f64) * t7624 * t17720;
    let t104695 = t26865 * t15904;
    let t104696 = t13127 * t104695;
    let t104703 = t17400 * t26866;
    let t104706 = sigma2 * t1802;
    let t104707 = t104706 * t3089;
    let t104708 = t3717 * t104707;
    (t104689, t104691, t104695, t104696, t104703, t104706, t104707, t104708)
}
