//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1013/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1013<F: Float>(t2297: F, t8901: F, t13364: F, t33944: F, t2001: F, t5574: F, t13287: F, t31195: F, t39891: F, t2302: F, t31057: F, t8406: F, t7447: F, t9701: F, t7440: F, t9705: F) -> (F, F, F, F, F, F, F) {
    let t40017 = t2297 * t8901;
    let t40019 = t33944 * t13364 * t40017;
    let t40029 = t2001 * t5574;
    let t40034 = t31195 * t13287 * t39891;
    let t40043 = t31057 * t13287 * t2302 * t8406;
    let t40045 = t7447 * t9701;
    let t40047 = t7440 * t9705;
    (t40017, t40019, t40029, t40034, t40043, t40045, t40047)
}
