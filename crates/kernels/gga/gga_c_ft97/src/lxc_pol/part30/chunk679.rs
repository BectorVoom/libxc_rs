//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 679/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk679<F: Float>(t193: F, t28836: F, t89: F, t25042: F, t25146: F, t25163: F, t25343: F, t25351: F, t28811: F, t28814: F, t28819: F, t28824: F, t28829: F, t28833: F) -> (F, F, F) {
    let t28837 = t193 * t28836;
    let t28838 = t89 * t28837;
    let t28840 = -t25343 - t25042 / F::cast_from(27.0_f64) + t25146 / F::cast_from(18.0_f64) - t25351 - t28811 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t28814 + t28819 / F::cast_from(12.0_f64) + t28824 / F::cast_from(12.0_f64) - t25163 / F::cast_from(54.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28829 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28833 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28838;
    (t28837, t28838, t28840)
}
