//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1197/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1197(t33: f64, t265: f64, t502: f64, t96072: f64, t10326: f64, t2085: f64, t2258: f64, t26666: f64, t57: f64, t606: f64, t7468: f64, t96121: f64, t96166: f64, t25876: f64, t26304: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t96168 = piecewise3(t503, 0.0_f64, t96072);
    let t96178 = piecewise3(t400, t96121 + t96166, t96168 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26666 * t606 - 3.0_f64 / 2.0_f64 * t7468 * t2258 - t2085 * t10326 / 2.0_f64);
    let t96186 = t25876 * t26304;
    (t96178, t96186)
}
