//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 455/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk455<F: Float>(t2654: F, t714: F, t1062: F, t723: F, t1697: F, t954: F, t1640: F, t219: F) -> (F, F, F, F) {
    let t2655 = t2654 * t714;
    let t2657 = t1062 * t723;
    let t2672 = t1697 * t954;
    let t2677 = t1640 * t219;
    (t2655, t2657, t2672, t2677)
}
