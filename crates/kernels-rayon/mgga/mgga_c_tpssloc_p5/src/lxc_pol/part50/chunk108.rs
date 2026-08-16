//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 108/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk108(t273: f64, t276: f64, t279: f64, t285: f64) -> (f64, f64, f64, f64, f64) {
    let t302 = 1.0_f64 + 0.5137e-1_f64 * t273;
    let t307 = 0.705945e1_f64 * t276 + 0.1549425e1_f64 * t273 + 0.420775e0_f64 * t279 + 0.1562925e0_f64 * t285;
    let t310 = 1.0_f64 + 0.32163958997385070134e2_f64 / t307;
    let t311 = f64::ln(t310);
    let t315 = 1.0_f64 + 0.278125e-1_f64 * t273;
    (t302, t307, t310, t311, t315)
}
