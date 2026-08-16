//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2289/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2289(t25: f64, t265: f64, t394: f64, t100624: f64, t1409: f64, t16558: f64, t2116: f64, t27373: f64, t29507: f64, t3966: f64, t40: f64, t5398: f64, t607: f64, t7274: f64, t7992: f64, t99069: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t103113 = piecewise3(t395, 0.0_f64, t100624);
    let t103125 = piecewise3(t115, t99069, t103113 * t40 / 2.0_f64 + t29507 * t607 / 2.0_f64 + t27373 * t1409 + t7992 * t3966 + t7274 * t5398 / 2.0_f64 + t2116 * t16558 / 2.0_f64);
    t103125
}
