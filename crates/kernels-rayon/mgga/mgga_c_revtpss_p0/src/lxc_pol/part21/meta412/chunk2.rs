//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1883/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1883(t33: f64, t265: f64, t502: f64, t11095: f64, t12562: f64, t13194: f64, t10326: f64, t1113: f64, t1304: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t504: f64, t57: f64, t606: f64, t895: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t13196 = piecewise3(t503, t12562 + t13194, t11095);
    let t13206 = piecewise3(t400, t11095 * t33 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2838 * t1113 + 3.0_f64 / 2.0_f64 * t895 * t3351 + t265 * t9357 / 2.0_f64, t13196 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t3805 * t606 - 3.0_f64 / 2.0_f64 * t1304 * t2258 - t504 * t10326 / 2.0_f64);
    (t13196, t13206)
}
