//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1098/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1098<F: Float>(t4872: F, t45304: F, t58969: F, t73956: F, t73958: F, t85518: F, t85526: F, t85533: F, t85540: F, t85548: F, t85556: F, t85560: F, t86744: F, t86747: F, t86750: F) -> (F, F) {
    let t87975 = t4872 * t4872;
    let t87994 = -F::cast_from(0.11554466666666666666e1_f64) * t73956 + F::cast_from(0.38514888888888888888e0_f64) * t73958 - F::cast_from(0.38514888888888888888e0_f64) * t58969 + F::cast_from(0.234754e0_f64) * t86750 - F::cast_from(0.352131e0_f64) * t86747 - F::cast_from(0.42794320987654320987e0_f64) * t85556 - F::cast_from(0.14443083333333333333e0_f64) * t85560 - F::cast_from(0.44016375e0_f64) * t86744 + F::cast_from(0.19257444444444444444e1_f64) * t85518 - F::cast_from(0.34663399999999999999e1_f64) * t85526 - F::cast_from(0.28886166666666666666e0_f64) * t85533 + F::cast_from(0.34663399999999999999e1_f64) * t85540 + F::cast_from(0.86658499999999999998e0_f64) * t85548 + F::cast_from(0.59912049382716049381e0_f64) * t45304;
    (t87975, t87994)
}
