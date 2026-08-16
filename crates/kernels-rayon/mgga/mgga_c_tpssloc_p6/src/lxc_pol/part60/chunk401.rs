//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 401/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk401(t3293: f64, t3030: f64, t466: f64, t3032: f64, t1208: f64, t476: f64) -> (f64, f64, f64, f64) {
    let t3464 = 5.0_f64 / 18.0_f64 * t3293;
    let t3499 = t466 * t3030;
    let t3500 = t3499 * t3032;
    let t3502 = 1.0_f64 / t1208 / t476;
    (t3464, t3499, t3500, t3502)
}
