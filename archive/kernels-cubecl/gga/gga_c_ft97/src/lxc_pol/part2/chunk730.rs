//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 730/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk730<F: Float>(t11472: F, t11473: F, t10965: F, t83: F, t1825: F, t3214: F, t452: F, t11430: F, t11432: F, t11436: F, t11439: F, t11444: F, t11448: F, t11451: F, t11455: F, t11459: F, t11463: F, t11467: F, t11469: F, t1901: F, t446: F) -> F {
    let t11474 = t11472 * t11473;
    let t11477 = t83 * t10965;
    let t11481 = t452 * t1825 * t3214;
    let t11484 = t11430 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11432 - t11436 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t11439 + t1901 * t11444 / F::cast_from(9.0_f64) - t11448 + t1901 * t11451 / F::cast_from(9.0_f64) - t446 * t11455 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11459 - t446 * t11463 / F::cast_from(3.0_f64) + t11467 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11469 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11474 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11477 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11481;
    t11484
}
