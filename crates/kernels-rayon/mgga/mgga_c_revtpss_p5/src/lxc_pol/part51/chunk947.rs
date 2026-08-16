//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 947/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk947(t30: f64, t265: f64, t393: f64, t1100: f64, t1102: f64, t198: f64, t32026: f64, t32030: f64, t32036: f64, t32058: f64, t336: f64, t5023: f64, t7177: f64, t7181: f64, t31882: f64, t45: f64, t606: f64, t8543: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t32059 = piecewise3(t394, t1102 * t198 * t32026 * t336 - t1100 * t32030 * t5023 + 2.0_f64 * t1100 * t32036 * t5023 - 2.0_f64 * t5023 * t7177 * t7181, t32058);
    let t32064 = piecewise3(t120, t31882, t32059 * t45 / 2.0_f64 + t8543 * t606 / 2.0_f64);
    (t32059, t32064)
}
