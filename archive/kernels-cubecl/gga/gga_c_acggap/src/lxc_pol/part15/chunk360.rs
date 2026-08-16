//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 360/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk360<F: Float>(t1708: F, t85: F, t87: F, t40: F, t495: F) -> (F, F, F, F) {
    let t1709 = t1708 * t85;
    let t1710 = F::cast_from(0.19751673498613801407e-1_f64) * t1709;
    let t1711 = t1708 * t87;
    let t1712 = t40 * t1711;
    let t1713 = t495 * t495;
    (t1710, t1711, t1712, t1713)
}
