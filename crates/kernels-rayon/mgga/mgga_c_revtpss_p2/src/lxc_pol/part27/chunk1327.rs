//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1327/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1327(t33: f64, t10326: f64, t2159: f64, t2258: f64, t27048: f64, t57: f64, t606: f64, t7677: f64, t94325: f64, t97508: f64, t10192: f64, t10260: f64, t10263: f64, t10416: f64, t118: f64, t2165: f64, t2322: f64, t2371: f64, t27056: f64, t27076: f64, t27079: f64, t569: f64, t649: f64, t651: f64, t670: f64, t7586: f64, t7591: f64, t7683: f64, t92724: f64, t92727: f64, t92731: f64, t92733: f64, t92736: f64, t94341: f64, t94348: f64, t94352: f64, t94355: f64, t96835: f64, t96858: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t97518 = piecewise3(t400, t94325, t97508 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t27048 * t606 - 3.0_f64 / 2.0_f64 * t7677 * t2258 - t2159 * t10326 / 2.0_f64);
    let t97525 = -6.0_f64 * t7586 * t10263 - 12.0_f64 * t2322 * t27076 + t96835 * t569 - t92724 - t92727 - t92731 - t92733 - t92736 - 2.0_f64 * t7586 * t10260 - 6.0_f64 * t651 * t27056 * t670 - 6.0_f64 * t651 * t7683 * t2371 - 6.0_f64 * t2322 * t27079 + t2165 * t10192 - t118 * (t96858 + t97518) - t94341 - 3.0_f64 * t649 * t27056 + t94348 - 6.0_f64 * t10416 * t7591 - t94352 - t94355;
    t97525
}
