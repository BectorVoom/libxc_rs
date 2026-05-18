//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 593/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk593<F: Float>(t1186: F, t2336: F, t89: F, t2857: F, t3691: F, t446: F, t1091: F, t824: F, t2665: F, t3700: F, t835: F, t18: F, t792: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4032 = t89 * t2336 * t1186;
    let t4034 = t2857 * t3691;
    let t4035 = t446 * t4034;
    let t4037 = t1091 * t824;
    let t4038 = t2665 * t4037;
    let t4039 = t446 * t4038;
    let t4041 = t835 * t3700;
    let t4042 = t446 * t4041;
    let t4044 = t792 * t18;
    (t4032, t4034, t4035, t4037, t4038, t4039, t4041, t4042, t4044)
}
