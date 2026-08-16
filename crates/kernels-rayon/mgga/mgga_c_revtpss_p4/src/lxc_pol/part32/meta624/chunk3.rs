//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1971/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1971(t102117: f64, t102120: f64, t102122: f64, t102129: f64, t102131: f64, t102133: f64, t102135: f64, t102139: f64, t109400: f64, t109404: f64, t109408: f64, t109413: f64, t109417: f64, t96206: f64) -> f64 {
    let t109423 = 0.25702851531048074406e-1_f64 * t109400 + 0.43368140941025997311e-1_f64 * t109404 + t102117 + 0.72280234901709995518e-2_f64 * t109408 + 0.96373646535613327359e-3_f64 * t102120 - 0.28912093960683998207e-1_f64 * t109413 - 0.26019841438354088051e-1_f64 * t102122 + t96206 + 0.54878743191129263322e-2_f64 * t109417 - t102129 + 0.4818682326780666368e-3_f64 * t102131 + 0.3427046870806409921e-2_f64 * t102133 - 0.45699670022203476294e-2_f64 * t102135 - 0.13009920719177044025e-2_f64 * t102139;
    t109423
}
