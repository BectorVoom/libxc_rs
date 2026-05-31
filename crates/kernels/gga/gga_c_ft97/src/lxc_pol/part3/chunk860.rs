//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 860/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk860<F: Float>(t3483: F, t925: F, t13220: F, t11593: F, t13040: F, t13042: F, t13049: F, t13062: F, t13075: F, t13084: F, t17195: F, t17200: F, t17204: F, t17208: F, t17357: F, t17360: F, t17362: F, t17366: F, t1901: F, t446: F) -> F {
    let t17369 = t925 * t3483;
    let t17370 = t13220 * t17369;
    let t17373 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17195 + t1901 * t17200 / F::cast_from(9.0_f64) - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1901 * t17204 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11593 * t17208 - t13040 - t13042 - t13049 + t13062 - t446 * t17357 / F::cast_from(3.0_f64) + t17360 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17362 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13075 + t13084 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17366 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t17370;
    t17373
}
