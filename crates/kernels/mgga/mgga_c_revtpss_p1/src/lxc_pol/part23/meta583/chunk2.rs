//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2210/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2210<F: Float>(t23481: F, t2908: F, t141: F, t23485: F, t930: F, t4573: F, t5825: F, t2850: F, t128: F) -> (F, F, F, F, F, F, F) {
    let t23492 = t2908 * t23481;
    let t23493 = t141 * t23492;
    let t23495 = t930 * t23485;
    let t23496 = t141 * t23495;
    let t23499 = t4573 * t5825;
    let t23500 = t2850 * t23499;
    let t23501 = t128 * t23500;
    (t23492, t23493, t23495, t23496, t23499, t23500, t23501)
}
