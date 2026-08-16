//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1740/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1740(t22856: f64, t22861: f64, t24058: f64, t24060: f64, t24061: f64, t26306: f64, t26310: f64, t26312: f64, t26314: f64, t26320: f64, t26324: f64, t22767: f64, t22780: f64, t22799: f64, t22805: f64, t24049: f64, t24050: f64, t26234: f64, t26236: f64, t26238: f64, t26240: f64, t26246: f64, t26249: f64, t26286: f64, t26290: f64, t26293: f64, t26295: f64, t26299: f64, t26303: f64, t27012: f64, t27019: f64, t27032: f64) -> f64 {
    let t27049 = t26306 / 192.0_f64 + t26310 / 384.0_f64 - t26312 / 768.0_f64 + t26314 / 192.0_f64 + 0.67287926823567318088e-4_f64 * t22856 + t24058 - t22861 + t24060 + t24061 + 0.80745512188280781706e-3_f64 * t26320 - 0.40372756094140390853e-3_f64 * t26324;
    let t27051 = t27012 - t26234 / 768.0_f64 - t26236 / 768.0_f64 - t26238 / 768.0_f64 + 5.0_f64 / 192.0_f64 * t26240 + t22767 + 0.67287926823567318088e-4_f64 * t26246 + t26249 / 768.0_f64 - t27019 + 0.28260929265898273597e-2_f64 * t22780 + t27032 + t22799 + 0.16956557559538964158e-1_f64 * t22805 - t24049 + t24050 + t26286 / 8.0_f64 + 0.16956557559538964158e-1_f64 * t26290 - 0.40372756094140390853e-3_f64 * t26293 + 0.28260929265898273597e-2_f64 * t26295 + 0.24223653656484234512e-2_f64 * t26299 + 0.24223653656484234512e-2_f64 * t26303 + t27049;
    t27051
}
