//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 675/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk675<F: Float>(t3189: F, t8506: F, t1780: F, t480: F, t3195: F, t11549: F, t11550: F, t11553: F, t11558: F, t11562: F, t11567: F, t11570: F, t11574: F, t11578: F, t1901: F, t3281: F, t446: F, t8227: F, t8229: F, t8233: F, t8235: F) -> (F,) {
    let t11584 = t8506 * t3189;
    let t11587 = t1780 * t480;
    let t11588 = t11587 * t3195;
    let t11591 = t11549 - 4.0 / 27.0 * t11550 + 4.0 / 27.0 * t1901 * t11553 + 4.0 / 27.0 * t1901 * t11558 + 2.0 / 3.0 * t446 * t11562 - t11567 + t446 * t11570 / 3.0 - 4.0 / 9.0 * t3281 * t11574 + 4.0 / 27.0 * t11578 - 2.0 / 27.0 * t8227 - 2.0 / 9.0 * t8229 - 8.0 / 81.0 * t8233 + 2.0 / 81.0 * t8235 + 4.0 / 9.0 * t1901 * t11584 - 4.0 / 27.0 * t1901 * t11588;
    (t11591,)
}
