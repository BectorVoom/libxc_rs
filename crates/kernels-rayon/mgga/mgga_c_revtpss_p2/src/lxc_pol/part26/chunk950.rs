//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 950/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk950(t30: f64, t265: f64, t393: f64, t11095: f64, t12198: f64, t12199: f64, t10326: f64, t1106: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, t9344: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t12201 = piecewise3(t394, t12198 + t12199, t11095);
    let t12211 = piecewise3(t120, t11095 * t30 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2838 * t605 + 3.0_f64 / 2.0_f64 * t895 * t2257 + t265 * t9344 / 2.0_f64, t12201 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t3340 * t606 + 3.0_f64 / 2.0_f64 * t1106 * t2258 + t395 * t10326 / 2.0_f64);
    t12211
}
