//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 857/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk857<F: Float>(t1029: F, t3237: F, t1020: F, t3228: F, t879: F, t1036: F, t174: F, t386: F, t387: F, t3646: F, t383: F) -> (F, F, F, F, F) {
    let t14243 = t3237 * t1029;
    let t14245 = t3228 * t1020;
    let t14255 = t879 * t879;
    let t14260 = 0.12862205435420921092e-2 * t1036 * t386 * t387 * t174 * t14255;
    let t14283 = t3646 * t383;
    (t14243, t14245, t14255, t14260, t14283)
}
