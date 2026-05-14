//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1377/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1377<F: Float>(t10164: F, t2328: F, t3069: F, t2197: F, t852: F, t10182: F, t2298: F, t898: F, t3070: F, t8214: F, t3033: F, t8004: F, t18796: F, t3740: F, t3766: F, t6205: F) -> (F, F, F, F, F, F, F, F) {
    let t27512 = 0.69263436422725855036e2 * t2328 * t10164;
    let t27513 = t3069 * t3069;
    let t27516 = 4.0 * t2197 * t27513 * t852;
    let t27519 = 0.35089341735807877242e1 * t898 * t10182 * t2298;
    let t27521 = 4.0 * t8214 * t3070;
    let t27523 = 2.0 * t3033 * t8004;
    let t27525 = 2.0 * t18796 * t3740;
    let t27527 = 1.0 * t6205 * t3766;
    (t27512, t27513, t27516, t27519, t27521, t27523, t27525, t27527)
}
