//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 634/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk634(t25: f64, t265: f64, t394: f64, t5669: f64, t5954: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t396: f64, t40: f64, t5397: f64, t5398: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t5955 = piecewise3(t395, t5954, t5669);
    let t5962 = piecewise3(t115, t5669 * t25 / 2.0_f64 + t1534 * t1408 + t265 * t5397 / 2.0_f64, t5955 * t40 / 2.0_f64 + t1642 * t1409 + t396 * t5398 / 2.0_f64);
    (t5955, t5962)
}
