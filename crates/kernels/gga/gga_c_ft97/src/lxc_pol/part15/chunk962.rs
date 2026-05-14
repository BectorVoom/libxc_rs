//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 962/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk962<F: Float>(t4872: F, t45304: F, t58969: F, t73956: F, t73958: F, t85518: F, t85526: F, t85533: F, t85540: F, t85548: F, t85556: F, t85560: F, t86744: F, t86747: F, t86750: F, t16832: F, t20049: F, t3359: F, t39546: F, t4466: F, t59002: F, t59007: F, t73975: F, t73977: F, t73983: F, t73985: F, t85522: F, t85529: F, t85536: F, t85544: F, t85551: F) -> (F, F, F) {
    let t87975 = t4872 * t4872;
    let t87994 = -0.11554466666666666666e1 * t73956 + 0.38514888888888888888e0 * t73958 - 0.38514888888888888888e0 * t58969 + 0.234754e0 * t86750 - 0.352131e0 * t86747 - 0.42794320987654320987e0 * t85556 - 0.14443083333333333333e0 * t85560 - 0.44016375e0 * t86744 + 0.19257444444444444444e1 * t85518 - 0.34663399999999999999e1 * t85526 - 0.28886166666666666666e0 * t85533 + 0.34663399999999999999e1 * t85540 + 0.86658499999999999998e0 * t85548 + 0.59912049382716049381e0 * t45304;
    let t88010 = t39546 - 0.77029777777777777776e0 * t73975 + 0.11554466666666666666e1 * t73977 - 0.51995099999999999998e1 * t85544 + 0.11554466666666666666e1 * t85551 - 0.9628722222222222222e0 * t85522 + 0.34663399999999999999e1 * t85529 - 0.38514888888888888888e0 * t85536 + 0.21397160493827160493e0 * t73983 + 0.19257444444444444444e0 * t73985 + 0.1056393e1 * t16832 * t4466 - 0.469508e0 * t3359 * t20049 - 0.25676592592592592592e0 * t59002 + 0.77029777777777777776e0 * t59007;
    (t87975, t87994, t88010)
}
