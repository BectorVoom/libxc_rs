//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 762/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk762<F: Float>(t32355: F, t432: F, t28: F, t89: F, t32325: F, t370: F, t27: F, t32114: F, t32118: F, t32123: F, t32328: F, t32332: F, t32336: F, t32341: F, t32345: F, t32349: F, t32353: F) -> (F, F, F, F, F) {
    let t32356 = t32355 * t432;
    let t32357 = t28 * t32356;
    let t32358 = t89 * t32357;
    let t32360 = t370 * t32325;
    let t32362 = t89 * t27 * t32360;
    let t32364 = t32114 + t32118 / F::cast_from(18.0_f64) + t32123 / F::cast_from(3.0_f64) - t32328 / F::cast_from(6.0_f64) - t32332 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t32336 - F::cast_from(2.0_f64) * t32341 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t32345 + t32349 + t32353 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t32358 - t32362 / F::cast_from(3.0_f64);
    (t32356, t32358, t32360, t32362, t32364)
}
