//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1982/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1982(t98258: f64, t98260: f64, t98269: f64, t94514: f64, t94520: f64, t94527: f64, t94530: f64, t94534: f64, t94537: f64, t94540: f64, t96341: f64, t96342: f64) -> f64 {
    let t102548 = 0.11433071498151929859e-3_f64 * t98258;
    let t102549 = 35.0_f64 / 108.0_f64 * t98260;
    let t102557 = 7.0_f64 / 36.0_f64 * t98269;
    let t102558 = -t102548 - t102549 - 7.0_f64 / 24.0_f64 * t94514 - 35.0_f64 / 54.0_f64 * t94520 - t96341 + t96342 - 0.24390552529390783699e-2_f64 * t94527 + 0.11433071498151929859e-3_f64 * t94530 - 0.57165357490759649295e-3_f64 * t94534 + 0.2032800112371413129e-4_f64 * t94537 - 0.14457274399185490174e-3_f64 * t94540 + t102557;
    t102558
}
