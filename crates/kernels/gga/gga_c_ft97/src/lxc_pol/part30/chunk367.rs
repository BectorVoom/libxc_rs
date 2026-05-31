//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 367/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk367<F: Float>(t2568: F, t6175: F, t242: F, t6116: F, t6133: F, t6113: F, t6122: F, t6126: F, t6130: F, t6138: F, t6142: F, t6146: F) -> (F, F, F, F) {
    let t6176 = t2568 * t6175;
    let t6177 = t242 * t6176;
    let t6181 = t6116 / F::cast_from(6.0_f64);
    let t6184 = t6133 / F::cast_from(3.0_f64);
    let t6187 = t6113 / F::cast_from(4.0_f64) + t6181 + t6122 / F::cast_from(6.0_f64) + t6126 - t6130 / F::cast_from(2.0_f64) + t6184 + t6138 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t6142 - t6146;
    (t6177, t6181, t6184, t6187)
}
