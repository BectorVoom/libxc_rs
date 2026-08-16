//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2986/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2986(t1042: f64, t1063: f64, t11703: f64, t16095: f64, t16196: f64, t3092: f64, t3127: f64, t3188: f64, t43313: f64, t4573: f64, t4578: f64, t4801: f64, t54419: f64, t54432: f64, t54435: f64, t54438: f64, t54440: f64, t54443: f64, t54446: f64, t54450: f64, t906: f64) -> f64 {
    let t54455 = -0.42874018118069736972e-3_f64 * t3127 * t1042 * t54419 * t906 + 0.85748036236139473944e-3_f64 * t16095 * t3092 * t4578 * t43313 - 0.71456696863449561621e-3_f64 * t16095 * t11703 * t4573 * t43313 - 0.11433071498151929859e-2_f64 * t54432 - 0.57165357490759649295e-3_f64 * t54435 - 0.28582678745379824648e-2_f64 * t54438 + 0.95275595817932748827e-3_f64 * t54440 + 0.47637797908966374414e-3_f64 * t54443 + 0.1270341277572436651e-2_f64 * t54446 - 0.85748036236139473944e-3_f64 * t3188 * t16196 - 0.28582678745379824648e-3_f64 * t1063 * t1042 * t4801 * t54450;
    t54455
}
