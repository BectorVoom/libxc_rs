//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2627/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2627(t18529: f64, t4889: f64, t1174: f64, t135: f64, t22034: f64, t15338: f64, t18409: f64, t3447: f64, t15320: f64, t15376: f64, t18427: f64, t18434: f64, t52058: f64, t64711: f64, t64713: f64, t64718: f64, t64730: f64, t64733: f64) -> f64 {
    let t73386 = t4889 * t18529;
    let t73389 = t1174 * t135 * t22034;
    let t73395 = t3447 * t15338 * t18409;
    let t73399 = -0.14814814814814814814e-2_f64 * t64711 + 0.29629629629629629628e-2_f64 * t64713 + 0.27777777777777777777e-3_f64 * t64718 - 0.14814814814814814814e-2_f64 * t64730 + 0.37037037037037037036e-3_f64 * t64733 + t52058 + 0.22222222222222222222e-2_f64 * t73386 - 0.27777777777777777777e-3_f64 * t73389 + 0.16666666666666666666e-2_f64 * t3447 * t15320 * t18427 + 0.27777777777777777777e-3_f64 * t73395 - 0.44444444444444444443e-2_f64 * t15376 * t18434;
    t73399
}
