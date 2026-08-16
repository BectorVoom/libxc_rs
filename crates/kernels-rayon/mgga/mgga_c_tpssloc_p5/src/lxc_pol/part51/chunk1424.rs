//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1424/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1424(t113941: f64, t115306: f64, t115308: f64, t115318: f64, t115331: f64, t120201: f64, t120209: f64, t120213: f64, t122121: f64, t122127: f64, t122131: f64, t122133: f64, t16030: f64, t8637: f64) -> f64 {
    let t122137 = -t115306 + 0.41123351671205660912e-2_f64 * t122121 + 0.41123351671205660912e-2_f64 * t115308 + 0.16449340668482264365e-1_f64 * t122127 + 0.16449340668482264365e-1_f64 * t122131 + t120201 - t113941 + 0.19190897446562641759e-1_f64 * t122133 - 0.82246703342411321824e-2_f64 * t115318 - t16030 * t8637 - t115331 + t120209 + t120213;
    t122137
}
