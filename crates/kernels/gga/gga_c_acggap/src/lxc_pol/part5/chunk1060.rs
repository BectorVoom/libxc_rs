//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1060/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1060<F: Float>(t1140: F, t5636: F, t330: F, t5787: F, t1801: F, t3570: F, t1165: F, t13081: F, t13088: F, t13090: F, t13851: F, t16553: F, t16557: F, t16563: F, t16569: F, t16575: F, t1884: F, t3196: F) -> (F,) {
    let t21433 = t1140 * t5636;
    let t21435 = t330 * t5787;
    let t21440 = t3570 * t1801;
    let t21442 = -0.16006300097412701803e-1 * t16553 + 0.85748036236139473944e-3 * t16557 + 0.34299214494455789578e-2 * t16563 + 0.17149607247227894789e-2 * t16569 + 0.51448821741683684366e-1 * t13851 * t1165 * t1884 * t3196 - 0.80031500487063509016e-2 * t16575 + 7.0 / 144.0 * t21433 - 7.0 / 144.0 * t21435 - 0.25724410870841842183e-2 * t13081 - 0.32012600194825403606e-1 * t13088 + 0.32012600194825403606e-1 * t13090 - 35.0 / 108.0 * t21440;
    (t21442,)
}
