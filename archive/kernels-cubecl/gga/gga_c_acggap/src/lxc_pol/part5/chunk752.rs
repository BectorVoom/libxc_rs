//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 752/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk752<F: Float>(t1055: F, t5645: F, t345: F, t1049: F, t1769: F, t1734: F, t301: F) -> (F, F, F, F) {
    let t5646 = t1055 * t5645;
    let t5647 = t345 * t5646;
    let t5649 = t1049 * t1769;
    let t5651 = t1734 * t301;
    (t5646, t5647, t5649, t5651)
}
