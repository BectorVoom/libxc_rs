//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2340/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2340(t24574: f64, t29813: f64, t1238: f64, t14980: f64, t1760: f64, t19213: f64, t19219: f64, t19225: f64, t24589: f64, t24601: f64, t24602: f64, t24615: f64, t27389: f64, t27406: f64, t27741: f64, t27784: f64, t27785: f64, t27830: f64, t3598: f64, t3966: f64, t5060: f64, t7283: f64, t7300: f64, t8002: f64, t8061: f64, t94358: f64, t94680: f64, t95863: f64, t95866: f64, t95884: f64, t95889: f64) -> f64 {
    let t104609 = t24574 * t29813;
    let t104631 = 4.0_f64 * t14980 * t8061 + 4.0_f64 * t1238 * t3598 * t27741 * t1760 + t95863 + t95866 + 4.0_f64 * t27830 * t5060 - 0.54831135561607547884e-2_f64 * t7283 * t94680 * t8002 - 0.91385225936012579807e-3_f64 * t104609 - 6.0_f64 * t27784 * t27785 * t19219 + 0.3289868133696452873e-1_f64 * t7283 * t7300 * t24615 * t19213 + t95889 + 0.54831135561607547884e-2_f64 * t24589 * t24601 * t24602 * t3966 * t1760 + 0.54831135561607547884e-2_f64 * t24589 * t94358 * t8002 + 0.14621636149762012769e-1_f64 * t27406 * t27389 + 24.0_f64 * t27784 * t95884 * t19225;
    t104631
}
