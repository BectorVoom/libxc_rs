//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1053/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1053(t33: f64, t265: f64, t502: f64, t25759: f64, t32498: f64, t27799: f64, t32505: f64, t1113: f64, t1962: f64, t32534: f64, t1940: f64, t2403: f64, t26425: f64, t26585: f64, t28472: f64, t32080: f64, t32487: f64, t32491: f64, t57: f64, t606: f64, t7200: f64, t7207: f64, t7432: f64, t8657: f64, t8677: f64, t8682: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t32553 = t25759 * t32498;
    let t32559 = t27799 * t32505;
    let t32561 = t1113 * t1962;
    let t32569 = piecewise3(t503, 0.0_f64, t32534);
    let t32574 = piecewise3(t400, 3.0_f64 / 2.0_f64 * t2403 * t8657 * t7200 + t1940 * t32487 * t33 / 2.0_f64 - t1940 * t32491 * t7207 / 2.0_f64 + t1940 * t8657 * t1113 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t32553 - t1940 * t26585 * t8677 / 2.0_f64 + t28472 * t32559 - t1940 * t7432 * t32561 / 2.0_f64 - t1940 * t7432 * t32080 / 2.0_f64, t32569 * t57 / 2.0_f64 - t8682 * t606 / 2.0_f64);
    (t32553, t32559, t32561, t32569, t32574)
}
