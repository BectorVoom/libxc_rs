//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 618/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk618(t331: f64, t3363: f64, t3364: f64, t3365: f64, t3368: f64, t3419: f64, t3455: f64, t3457: f64, t3458: f64, t3461: f64, t3511: f64, t748: f64) -> f64 {
    let t3513 = t331 * t3455 - t3511 * t748 - t3363 + t3364 + t3365 - t3368 + t3419 - t3457 - t3458 + t3461;
    t3513
}
