//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 825/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk825(t574: f64, t5869: f64, t5935: f64, t33133: f64, t33138: f64, t33142: f64, t33146: f64, t33147: f64, t33151: f64, t33155: f64, t33157: f64, t33161: f64, t33163: f64, t33167: f64, t446: f64) -> (f64, f64) {
    let t33171 = t574 * t5935 * t5869;
    let t33174 = -2.0_f64 * t446 * t33133 - 2.0_f64 / 3.0_f64 * t446 * t33138 + 4.0_f64 / 3.0_f64 * t446 * t33142 + t33146 - t446 * t33147 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t33151 - t33155 + 2.0_f64 / 3.0_f64 * t446 * t33157 - t33161 - 2.0_f64 / 3.0_f64 * t446 * t33163 - t446 * t33167 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t33171;
    (t33171, t33174)
}
