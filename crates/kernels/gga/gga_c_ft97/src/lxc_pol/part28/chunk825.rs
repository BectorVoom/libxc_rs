//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 825/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk825<F: Float>(t574: F, t5869: F, t5935: F, t33133: F, t33138: F, t33142: F, t33146: F, t33147: F, t33151: F, t33155: F, t33157: F, t33161: F, t33163: F, t33167: F, t446: F) -> (F, F) {
    let t33171 = t574 * t5935 * t5869;
    let t33174 = -F::cast_from(2.0_f64) * t446 * t33133 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33138 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t33142 + t33146 - t446 * t33147 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33151 - t33155 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33157 - t33161 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33163 - t446 * t33167 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33171;
    (t33171, t33174)
}
