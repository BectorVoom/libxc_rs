//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 892/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk892<F: Float>(t319: F, t35972: F, t840: F, t1508: F, t2862: F, t7036: F, t10683: F, t35828: F, t34124: F, t34156: F, t34158: F, t36204: F, t36208: F, t36211: F, t36215: F, t36220: F, t36224: F, t446: F) -> (F, F, F, F) {
    let t36228 = t840 * t319 * t35972;
    let t36232 = t2862 * t1508 * t7036;
    let t36236 = t10683 * t319 * t35828;
    let t36239 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36204 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36208 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t36211 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36215 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36220 + t34124 + t34156 - t34158 - t446 * t36224 / F::cast_from(3.0_f64) - t446 * t36228 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t36232 - F::cast_from(2.0_f64) * t446 * t36236;
    (t36228, t36232, t36236, t36239)
}
