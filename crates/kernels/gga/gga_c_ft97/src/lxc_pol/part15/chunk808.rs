//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 808/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk808<F: Float>(t21373: F, t801: F, t10883: F, t13538: F, t18096: F, t18107: F, t18115: F, t21353: F, t21357: F, t21360: F, t21364: F, t21367: F, t21371: F, t21821: F, t4068: F, t4977: F) -> (F, F) {
    let t21825 = t801 * t21373;
    let t21837 = F::new(0.1760655e0) * t21821 - F::new(0.352131e0) * t4068 * t4977 + F::new(0.234754e0) * t21825 - t10883 - F::cast_from(0.19257444444444444444e0_f64) * t13538 + F::cast_from(0.9628722222222222222e-1_f64) * t18096 - F::cast_from(0.28886166666666666666e0_f64) * t18107 + F::cast_from(0.14443083333333333333e0_f64) * t18115 - F::cast_from(0.1604787037037037037e0_f64) * t21353 + F::cast_from(0.57772333333333333332e0_f64) * t21357 - F::cast_from(0.28886166666666666666e0_f64) * t21360 - F::cast_from(0.86658499999999999998e0_f64) * t21364 + F::cast_from(0.86658499999999999998e0_f64) * t21367 - F::cast_from(0.14443083333333333333e0_f64) * t21371;
    (t21825, t21837)
}
