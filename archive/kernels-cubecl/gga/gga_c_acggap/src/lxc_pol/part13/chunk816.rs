//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 816/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk816<F: Float>(t1980: F, t8859: F, t1967: F, t2304: F, t1507: F, t570: F, t1503: F, t2041: F, t1165: F, t1411: F, t604: F, t2068: F) -> (F, F, F, F, F, F) {
    let t8860 = t1980 * t8859;
    let t8862 = t1967 * t2304;
    let t8864 = t570 * t1507;
    let t8866 = t2041 * t1503;
    let t8869 = t1165 * t604 * t1411;
    let t8870 = t2068 * t8869;
    (t8860, t8862, t8864, t8866, t8869, t8870)
}
