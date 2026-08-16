//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1129/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1129(t39642: f64, t39721: f64, t39723: f64, t39816: f64, t39846: f64, t39882: f64, t39906: f64, t39977: f64, t40070: f64, t40109: f64, t40137: f64, t40220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41480 = 0.11708928647259339622e0_f64 * t39642;
    let t41518 = 0.57829097596741960691e-3_f64 * t39721;
    let t41519 = 0.16262400898971305031e-3_f64 * t39723;
    let t41570 = 0.11902492299418487743e0_f64 * t39816;
    let t41582 = 0.84755945902752848174e0_f64 * t39846;
    let t41600 = 0.45022119329691164871e0_f64 * t39882;
    let t41609 = 0.13506635798907349462e1_f64 * t39906;
    let t41641 = 0.42683466926433871473e0_f64 * t39977;
    let t41680 = 0.11902492299418487743e0_f64 * t40070;
    let t41699 = 0.84755945902752848174e0_f64 * t40109;
    let t41711 = 0.84755945902752848174e0_f64 * t40137;
    let t41750 = 0.45022119329691164871e0_f64 * t40220;
    (t41480, t41518, t41519, t41570, t41582, t41600, t41609, t41641, t41680, t41699, t41711, t41750)
}
