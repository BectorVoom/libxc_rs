//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1363/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1363(t34310: f64, t34239: f64, t4391: f64, t6964: f64, t34276: f64, t34279: f64, t34282: f64, t34285: f64, t34288: f64, t34291: f64, t34294: f64, t34297: f64, t34299: f64, t34301: f64, t34303: f64, t34305: f64, t34307: f64, t34309: f64) -> f64 {
    let t34311 = 0.59584149919750711116e-1_f64 * t34310;
    let t34314 = 0.85801175884441024006e1_f64 * t4391 * t6964 * t34239;
    let t34315 = t34276 - t34279 - t34282 + t34285 - t34288 + t34291 - t34294 + t34297 + t34299 + t34301 + t34303 + t34305 - t34307 - t34309 - t34311 + t34314;
    t34315
}
