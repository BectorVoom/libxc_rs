//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3258/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3258(t10811: f64, t18446: f64, t50600: f64, t50602: f64, t50604: f64, t50606: f64, t50608: f64, t50611: f64, t50615: f64, t50619: f64, t50628: f64, t50632: f64) -> f64 {
    let t61817 = t10811 * t18446;
    let t61829 = -0.80031500487063509015e-2_f64 * t61817 - 0.16006300097412701803e-1_f64 * t50600 - 0.80031500487063509016e-2_f64 * t50602 - 0.21683201198628406709e-2_f64 * t50604 - 0.22675591804667994222e-1_f64 * t50606 + 0.90702367218671976886e-1_f64 * t50608 + 0.16264433699083676445e-3_f64 * t50611 - 0.4065600224742826258e-4_f64 * t50615 - 0.20328001123714131289e-4_f64 * t50619 + 0.2032800112371413129e-3_f64 * t50628 - 0.50820002809285328224e-4_f64 * t50632;
    t61829
}
