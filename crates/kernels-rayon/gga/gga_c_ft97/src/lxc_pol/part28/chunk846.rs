//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 846/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk846(t34511: f64, t488: f64, t10969: f64, t7274: f64, t32066: f64, t32093: f64, t34373: f64, t34377: f64, t34382: f64, t34387: f64, t34391: f64, t34395: f64, t34399: f64, t34403: f64, t34408: f64) -> (f64, f64, f64) {
    let t34512 = t488 * t34511;
    let t34514 = t10969 * t7274;
    let t34524 = t34373 / 2.0_f64 + t32066 + 2.0_f64 / 9.0_f64 * t34377 + 4.0_f64 / 3.0_f64 * t34382 - 2.0_f64 / 3.0_f64 * t34387 - t34391 / 6.0_f64 - t32093 - t34395 / 9.0_f64 - t34399 + 2.0_f64 / 3.0_f64 * t34403 + t34408 / 12.0_f64;
    (t34512, t34514, t34524)
}
