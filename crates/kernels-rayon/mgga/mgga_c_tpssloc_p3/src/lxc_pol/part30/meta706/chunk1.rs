//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2321/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2321(t25: f64, t265: f64, t394: f64, t100578: f64, t100623: f64, t100528: f64, t1409: f64, t16558: f64, t1965: f64, t25883: f64, t28756: f64, t3966: f64, t40: f64, t5398: f64, t607: f64, t6835: f64, t7643: f64, t99069: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t100624 = t100578 + t100623;
    let t100625 = piecewise3(t395, t100528, t100624);
    let t100637 = piecewise3(t115, t99069, t100625 * t40 / 2.0_f64 + t28756 * t607 / 2.0_f64 + t25883 * t1409 + t7643 * t3966 + t6835 * t5398 / 2.0_f64 + t1965 * t16558 / 2.0_f64);
    (t100624, t100637)
}
