//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 421/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk421<F: Float>(t2469: F, t766: F, t242: F, t250: F, t251: F, t747: F, t91: F, t1771: F, t249: F, t1775: F, t740: F, t458: F, t745: F, t2344: F, t241: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2470 = t2469 * t766;
    let t2471 = t242 * t2470;
    let t2475 = 1.0 / t251 / t250;
    let t2476 = t747 * t747;
    let t2478 = t91 * t2475 * t2476;
    let t2481 = 4.0 / 9.0 * t1771 * t249;
    let t2482 = t1775 * t740;
    let t2484 = t458 * t745;
    let t2486 = t2344 * t241;
    (t2470, t2471, t2475, t2476, t2478, t2481, t2482, t2484, t2486)
}
