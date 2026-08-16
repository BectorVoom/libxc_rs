//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1768/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1768(t1310: f64, t1453: f64, t2014: f64, t2322: f64, t25082: f64, t28652: f64, t28704: f64, t28707: f64, t28709: f64, t28711: f64, t28718: f64, t28727: f64, t4248: f64, t4254: f64, t4293: f64, t4297: f64, t508: f64, t649: f64, t651: f64, t7359: f64, t7378: f64, t7969: f64, t7984: f64, t8065: f64, t8075: f64) -> f64 {
    let t28729 = -t1310 * t7969 + t1453 * t8075 - t2014 * t28707 - t2014 * t28709 - t2014 * t28727 - 2.0_f64 * t2322 * t7984 - 3.0_f64 * t25082 * t28718 - t28652 * t508 - 2.0_f64 * t28704 * t651 - 2.0_f64 * t28711 * t651 - 2.0_f64 * t4248 * t7378 - 2.0_f64 * t4254 * t7984 - 2.0_f64 * t4293 * t7359 - 2.0_f64 * t4297 * t7359 - t649 * t8065;
    t28729
}
