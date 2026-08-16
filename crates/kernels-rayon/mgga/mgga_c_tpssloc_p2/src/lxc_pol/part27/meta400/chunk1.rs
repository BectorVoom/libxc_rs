//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1664/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1664(t1760: f64, t3630: f64, t3598: f64, t1238: f64, t1252: f64, t14972: f64, t14980: f64, t15787: f64, t15790: f64, t15794: f64, t15797: f64, t15800: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t498: f64, t5055: f64, t5060: f64, t5089: f64) -> f64 {
    let t15802 = t1760 * t3630;
    let t15803 = t3598 * t15802;
    let t15806 = -t1238 * t15787 + 4.0_f64 * t1238 * t15790 - 6.0_f64 * t1238 * t15794 + 2.0_f64 * t1238 * t15803 - 2.0_f64 * t1252 * t14972 - 2.0_f64 * t1252 * t14980 - 2.0_f64 * t1252 * t15797 + t15800 * t498 + 4.0_f64 * t3487 * t5060 - 2.0_f64 * t3593 * t5089 + 2.0_f64 * t3600 * t5055 - t3631 * t5055;
    t15806
}
