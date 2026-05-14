//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 321/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk321<F: Float>(t1049: F, t503: F, t1055: F, t1427: F, t345: F, t355: F, t495: F) -> (F, F, F, F) {
    let t1474 = t1049 * t503;
    let t1476 = t1055 * t1427;
    let t1477 = t345 * t1476;
    let t1479 = t355 * t495;
    (t1474, t1476, t1477, t1479)
}
