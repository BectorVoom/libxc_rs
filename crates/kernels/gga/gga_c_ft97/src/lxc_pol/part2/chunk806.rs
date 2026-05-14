//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 806/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk806<F: Float>(t13780: F, t13794: F, t13809: F, t13811: F, t13759: F, t13775: F, t13778: F, t13783: F, t13786: F, t13789: F, t13792: F, t13798: F, t13801: F, t13804: F, t13807: F, t13814: F, t13817: F, t13820: F, t13823: F, t9699: F) -> (F,) {
    let t14336 = t13780 / 27.0;
    let t14341 = 2.0 / 81.0 * t13794;
    let t14346 = t13809 / 27.0;
    let t14347 = 2.0 / 27.0 * t13811;
    let t14352 = -2.0 / 9.0 * t13759 + t13775 / 18.0 + t13778 / 27.0 - t14336 + t13783 / 9.0 - 2.0 / 9.0 * t13786 + t13789 / 9.0 - 4.0 / 9.0 * t13792 + t14341 - t13798 / 27.0 - 5.0 / 81.0 * t13801 + 4.0 / 27.0 * t13804 + t13807 / 18.0 - t14346 - t9699 - t14347 - t13814 / 9.0 - t13817 / 3.0 + 2.0 / 9.0 * t13820 - t13823 / 9.0;
    (t14352,)
}
