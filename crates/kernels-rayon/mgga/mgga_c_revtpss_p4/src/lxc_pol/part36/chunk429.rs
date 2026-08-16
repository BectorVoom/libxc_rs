//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 429/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk429(t33: f64, t265: f64, t502: f64, t2144: f64, t2149: f64, t2152: f64, t460: f64, t1300: f64, t198: f64, t1995: f64, t336: f64, t2002: f64, t57: f64, t2132: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t2155 = 0.65854491829355115987e0_f64 * t460 * t2144 - 0.4336814094102599731e0_f64 * t2149 * t2152;
    let t2159 = piecewise3(t503, t198 * t336 * t2155 * t1300, t1995);
    let t2162 = piecewise3(t400, t2002, t2159 * t57 / 2.0_f64);
    let t2163 = t2132 + t2162;
    (t2155, t2159, t2163)
}
