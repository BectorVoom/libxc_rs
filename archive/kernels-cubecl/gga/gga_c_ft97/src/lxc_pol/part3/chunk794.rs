//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 794/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk794<F: Float>(t16320: F, t8557: F, t3219: F, t925: F, t11854: F, t16312: F, t2992: F, t11472: F, t11882: F, t11883: F, t16288: F, t16293: F, t16296: F, t16298: F, t16300: F, t16302: F, t16306: F, t16309: F, t16314: F, t16317: F, t1901: F, t446: F) -> F {
    let t16321 = t8557 * t16320;
    let t16324 = t925 * t3219;
    let t16325 = t11854 * t16324;
    let t16328 = t2992 * t16312;
    let t16329 = t11472 * t16328;
    let t16332 = t446 * t16288 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16293 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16296 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t16298 + t16300 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16302 + t11882 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t11883 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16306 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t16309 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t16314 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t16317 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16321 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t16325 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t16329;
    t16332
}
