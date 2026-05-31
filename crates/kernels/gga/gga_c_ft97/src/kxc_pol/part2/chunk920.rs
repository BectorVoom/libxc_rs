//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 920/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk920<F: Float>(t3972: F, t713: F, t729: F, t762: F, t10048: F, t10062: F, t10064: F, t10090: F, t14212: F, t14215: F, t14219: F, t14223: F, t14224: F, t14228: F, t14232: F, t14233: F, t14240: F, t14242: F, t3281: F, t446: F) -> F {
    let t14245 = t3972 * t713;
    let t14247 = t729 * t762 * t14245;
    let t14251 = t14212 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t14215 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3281 * t14219 - t14223 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t14224 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t14228 + t14232 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14233 + t10048 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10062 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10064 - t14240 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t14242 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t14247 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10090;
    t14251
}
