//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 850/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk850(t2574: f64, t265: f64, t35323: f64, t10157: f64, t35318: f64, t33291: f64, t33318: f64, t35312: f64, t35316: f64, t35321: f64, t35326: f64, t35330: f64, t35334: f64, t35338: f64, t35341: f64, t35346: f64) -> (f64, f64, f64) {
    let t35653 = t2574 * t265 * t35323;
    let t35657 = t10157 * t265 * t35318;
    let t35669 = 3.0_f64 / 2.0_f64 * t35312 + t33291 + 2.0_f64 / 3.0_f64 * t35316 + 4.0_f64 * t35321 - 2.0_f64 * t35326 - t35330 / 2.0_f64 - t33318 - t35334 / 3.0_f64 - 3.0_f64 * t35338 + 2.0_f64 * t35341 + t35346 / 4.0_f64;
    (t35653, t35657, t35669)
}
