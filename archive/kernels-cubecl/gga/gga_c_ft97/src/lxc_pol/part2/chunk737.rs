//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 737/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk737<F: Float>(t3189: F, t8506: F, t1780: F, t480: F, t3195: F, t11549: F, t11550: F, t11553: F, t11558: F, t11562: F, t11567: F, t11570: F, t11574: F, t11578: F, t1901: F, t3281: F, t446: F, t8227: F, t8229: F, t8233: F, t8235: F) -> F {
    let t11584 = t8506 * t3189;
    let t11587 = t1780 * t480;
    let t11588 = t11587 * t3195;
    let t11591 = t11549 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11550 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t11553 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t11558 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11562 - t11567 + t446 * t11570 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3281 * t11574 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11578 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8227 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8229 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t8233 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t8235 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11584 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t11588;
    t11591
}
