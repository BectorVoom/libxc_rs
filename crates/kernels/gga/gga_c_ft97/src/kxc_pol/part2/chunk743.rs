//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 743/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk743<F: Float>(t11392: F, t24: F, t469: F, t3155: F, t458: F, t1771: F, t963: F, t358: F, t378: F, t93: F, t1587: F, t1755: F, t3149: F) -> (F, F, F, F, F) {
    let t11665 = t24 * t469 * t11392;
    let t11668 = F::new(2.0) / F::new(3.0) * t458 * t3155;
    let t11669 = t1771 * t963;
    let t11672 = t378 * t93 * t358;
    let t11676 = t1587 * t3149 * t1755;
    (t11665, t11668, t11669, t11672, t11676)
}
