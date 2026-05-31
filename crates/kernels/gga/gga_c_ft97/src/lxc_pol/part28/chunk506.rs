//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 506/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk506<F: Float>(t1736: F, t70: F, t2252: F, t342: F, t511: F, t1526: F, t1944: F, t7705: F, t1948: F, t630: F, t128: F, t39: F) -> (F, F, F, F, F) {
    let t8633 = t70 * t1736;
    let t8759 = t342 * t2252 * t511 / F::cast_from(18.0_f64);
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    let t8811 = t128 * t39;
    (t8633, t8759, t8761, t8764, t8811)
}
