//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1405/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1405(t33: f64, t265: f64, t502: f64, t18884: f64, t20691: f64, t21643: f64, t1113: f64, t1304: f64, t1469: f64, t1711: f64, t18281: f64, t1837: f64, t18892: f64, t20256: f64, t4186: f64, t4560: f64, t504: f64, t5509: f64, t57: f64, t5825: f64, t606: f64, t6084: f64, t6416: f64, t6757: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t21645 = piecewise3(t503, t20691 + t21643, t18884);
    let t21657 = piecewise3(t400, t18884 * t33 / 2.0_f64 + t6084 * t1113 / 2.0_f64 + t4560 * t1711 - t18892 + t895 * t6416 / 2.0_f64 + t265 * t20256 / 2.0_f64, t21645 * t57 / 2.0_f64 - t6757 * t606 / 2.0_f64 - t5509 * t1469 - t1837 * t4186 - t1304 * t5825 / 2.0_f64 - t504 * t18281 / 2.0_f64);
    t21657
}
